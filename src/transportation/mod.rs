mod transportation_models;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::thread;

use chrono::{Local, TimeDelta, Utc};
use graphql_client::{GraphQLQuery, Response};
use log::info;
use log::kv::Source;
use reqwest;
use reqwest::header;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel, Weak};
use query::Variables;
use crate::StaticAssets;
use crate::transportation::query::{QueryStopPlace, QueryStopPlaceQuays, QueryStopPlaceQuaysEstimatedCalls, ResponseData, TransportMode};
use crate::transportation::transportation_models::{TrackedQuay, TrackedStop, TrackedStops};
use crate::ui::{MainWindow, StopPlaceData, StopPlaceDataRow};

const BASE_URL: &str = "https://api.entur.io/journey-planner/v3/graphql";
// https://developer.entur.org/pages-intro-authentication
const ET_CLIENT: &str = "knowit-objectnet-trd-infoscreen";
const PRINSENS_GATE: &str = "NSR:StopPlace:41613";
const PRINSENS_GATE_P1: &str = "NSR:Quay:71184";
const PRINSENS_GATE_P2: &str = "NSR:Quay:71181";
const DEFAULT_TIME_RANGE: i64 = 72100;
const DEFAULT_NUMBER_OF_DEPARTURES: i64 = 10;

type Date = String;
type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/transportation/journey-planner_schema.json",
    query_path = "src/transportation/journey-planner_query.graphql",
    response_derives = "Debug, Clone"
)]
struct Query;

pub fn setup(window: &MainWindow) {
    let window_weak = window.as_weak();
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(transportation_worker_loop(window_weak))
    });
}

async fn transportation_worker_loop(window: Weak<MainWindow>) {
    loop {
        match read_tracked_stops() {
            Ok(tracked_stops) => {
                let all_stops_data: VecModel<StopPlaceData> = VecModel::default();
                
                for tracked_stop in tracked_stops.stops {
                    // 1. Run GraphQL Query
                    let result = get_stop_place(tracked_stop.clone()).await;
                    let stop_place = match result {
                        Ok(response) => match response.stop_place {
                            Some(stop_place) => stop_place,
                            None => {
                                info!("No stop places returned from server. Check if the StopPlace-ID exists.");
                                continue
                            }
                        }
                        Err(_) => {
                            info!("Error from server. Check query.");
                            continue
                        }
                    };
                    
                    // 2. Filter on tracked quays and lines
                    let filtered_quays = match filter_by_quays_and_lines(stop_place, tracked_stop) {
                        None => {
                            info!("No quays left after filtering. Check if Quay-IDs exist on this StopPlace.");
                            continue
                        },
                        Some(filtered_quays) => filtered_quays
                    };
                    
                    // 3. Construct StopPlaceData and place in Vec
                    for quay in filtered_quays {
                        let stop_place_data_rows: VecModel<StopPlaceDataRow> = VecModel::default();
                        
                        for estimated_call in quay.estimated_calls {
                            match extract_relevant_values(estimated_call) {
                                None => {
                                    info!("Unable to extract values from estimated call. Skipping.");
                                    continue
                                }
                                Some(row) => stop_place_data_rows.push(row) 
                            };
                        }
                        
                        let stop_place_data = StopPlaceData {
                            stopName: SharedString::from(format!("{} {}", quay.name, quay.public_code.unwrap_or(String::from("")))),
                            stopDataRows: Rc::new(stop_place_data_rows).into()
                        };

                        all_stops_data.push(stop_place_data);
                    }
                }
                
                // Display Vec of StopPlaceData
                display_transportation(&window, all_stops_data);
            }
            Err(_) => {
                info! { "Unable to read file containing tracked stops information." }
            }
        };
        
        tokio::time::sleep(std::time::Duration::from_secs(60 * 60)).await;
    }
}

fn display_transportation(window_weak: &Weak<MainWindow>, all_stops_data: VecModel<StopPlaceData>) {
    /*
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            window.set_stopPlacesData(Rc::new(all_stops_data).into())
        })
        .unwrap();
    */
}

fn read_tracked_stops() -> Result<TrackedStops, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open("src/transportation/trackedStops.json")?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let tracked_stops = serde_json::from_reader(reader)?;

    // Return the object.
    Ok(tracked_stops)
}

fn filter_by_quays_and_lines(query_stop_place: QueryStopPlace, tracked_stop: TrackedStop) -> Option<Vec<QueryStopPlaceQuays>> {
    let all_query_quays: Vec<&QueryStopPlaceQuays> = match query_stop_place.quays {
        None => {
            info!("No quays for stop with ID {}", query_stop_place.id);
            return None
        },
        Some(quays) => quays.iter().flatten().collect::<Vec<&QueryStopPlaceQuays>>()
    };
    
    let tracked_quays: Vec<&TrackedQuay> = tracked_stop.quays.iter().flatten().collect();
    let mut picked_query_quays: Vec<QueryStopPlaceQuays> = Vec::new();
    
    for tracked_quay in tracked_quays {
        for query_quay in all_query_quays {
            if tracked_quay.id == query_quay.id {
                picked_query_quays.push(query_quay.clone())
            }
        }
    }
    
    // TODO:
    // - Implement filtering by specific lines per quay

    return Some(picked_query_quays)
}

fn extract_relevant_values(estimated_call: QueryStopPlaceQuaysEstimatedCalls) -> Option<StopPlaceDataRow> {
    let transport_mode = match &estimated_call.service_journey.journey_pattern {
        None => {
            info!("No journey pattern found for service journey '{}'", estimated_call.service_journey.public_code.unwrap_or(String::from("unknown")));
            return None;
        },
        Some(journey_pattern) => {
            match &journey_pattern.line.transport_mode {
                None => {
                    info!("No transport mode found for line '{}'", journey_pattern.line.id);
                    return None
                },
                Some(transport_mode) => {
                    transport_mode
                }
            }
        }
    };

    // TODO: 
    // - More error handling and less unwrapping.
    
    let public_code = estimated_call
        .service_journey.journey_pattern.as_ref().unwrap()
        .line.public_code.as_ref().unwrap();

    let destination_front_text = estimated_call
        .destination_display.as_ref().unwrap()
        .front_text.as_ref().unwrap();

    let aimed_departure_time = estimated_call.aimed_departure_time;

    let custom_format = match format_departure_time(aimed_departure_time) {
        None => {
            info!("Unable to format departure time: {}", aimed_departure_time.to_string());
            return None
        },
        Some(value) => value,
    };

    let stop_place_data_row = StopPlaceDataRow {
        transportMode: get_icon(transport_mode.as_str()),
        publicCode: SharedString::from(public_code),
        destinationFrontText: SharedString::from(destination_front_text),
        aimedDepartureTime: SharedString::from(custom_format),
    };
    
    return Some(stop_place_data_row)
}

async fn get_stop_place(tracked_stop: TrackedStop) -> Result<ResponseData, Box<dyn Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", header::HeaderValue::from_static("application/json"));
    headers.insert("ET-Client-Name",header::HeaderValue::from_static(ET_CLIENT));

    let variables = Variables {
        id: tracked_stop.id,
        time_range: Some(DEFAULT_TIME_RANGE),
        number_of_departures: Some(DEFAULT_NUMBER_OF_DEPARTURES),
    };
    
    let request_body = Query::build_query(variables);
    
    let client = reqwest::Client::new();

    let res = client.post(BASE_URL)
        .headers(headers)
        .json(&request_body)
        .send().await?;
    
    let response_body: Response<ResponseData> = res.json().await?;

    println!("{:#?}", response_body);

    Ok(response_body.data.expect("Missing response data"))
}

fn format_departure_time(departure_time: DateTime) -> Option<String> {
    let local_now = Local::now();
    let local_departure_time = departure_time.with_timezone(&Local);

    let time_delta = local_departure_time.signed_duration_since(local_now);

    return
    if time_delta < TimeDelta::zero() {
        None
    } else if time_delta < TimeDelta::minutes(1) {
        Some(String::from("Nå"))
    } else if time_delta < TimeDelta::minutes(10) {
        Some(format!("{} min", time_delta.num_minutes()))
    } else {
        Some(format!("{}", local_departure_time.format("%H:%M")))
    }
}

fn get_icon(icon_name: &str) -> Image {
    let icon_path = std::format!("transport/{}.png", icon_name);
    let icon_data = match StaticAssets::get(&icon_path) {
        Some(icon_data) => icon_data.data.into_owned(),
        None => StaticAssets::get("not-found.png")
            .unwrap()
            .data
            .into_owned(),
    };

    let transport_icon = image::load_from_memory_with_format(&icon_data, image::ImageFormat::Png)
        .unwrap()
        .into_rgba8();

    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        transport_icon.as_raw(),
        transport_icon.width(),
        transport_icon.height(),
    );

    Image::from_rgba8(buffer)
}


impl TransportMode {
    fn as_str(&self) -> &'static str {
        match self {
            TransportMode::bus => "bus",
            TransportMode::tram => "tram",
            TransportMode::rail => "rail",
            _ => "unknown"
        }
    }
}