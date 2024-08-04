use std::error::Error;
use std::rc::Rc;
use std::thread;

use chrono::{Local, TimeDelta, Utc};
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use reqwest::header;
use slint::{ComponentHandle, Image, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel, Weak};
use query::Variables;
use crate::StaticAssets;
use crate::transportation::query::{QueryStopPlace, QueryStopPlaceQuays, ResponseData, TransportMode};
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
    schema_path = "src/resources/journey-planner_schema.json",
    query_path = "src/resources/journey-planner_query.graphql",
    response_derives = "Debug"
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
        let result = get_stop_place().await;
        if result.is_ok() {
            let option_stop_place = result.unwrap().stop_place;
            if option_stop_place.is_some() {
                let stop_place = option_stop_place.unwrap();
                display_transportation(&window, stop_place)
            }
        } 
        tokio::time::sleep(std::time::Duration::from_secs(60 * 60)).await;
    }
}

fn display_transportation(window_weak: &Weak<MainWindow>, stop_place: QueryStopPlace) {
    
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {

            let stop_place_data_rows: VecModel<StopPlaceDataRow> = VecModel::default();
            
            let quays: Vec<QueryStopPlaceQuays> = match stop_place.quays {
                None => panic!("No quays for this stop."),
                Some(quays) => quays.into_iter().map(|quay| match quay {
                    None => panic!("No data in Quay."),
                    Some(quay) => quay
                }).collect()
            };
            
            let filtered_quays_option = quays
                .into_iter()
                .find(|quay| quay.id == PRINSENS_GATE_P1);
            
            let quay = match filtered_quays_option {
                None => panic!("No quay with this ID."),
                Some(quay) => quay
            };
            
            for estimated_call in quay.estimated_calls {
                
                let transport_mode = match &estimated_call.service_journey.journey_pattern {
                    None => continue,
                    Some(journey_pattern) => {
                        match &journey_pattern.line.transport_mode {
                            None => continue,
                            Some(transport_mode) => {
                                transport_mode
                            }
                        }
                    }
                };
                

                let public_code = estimated_call
                    .service_journey.journey_pattern.as_ref().unwrap()
                    .line.public_code.as_ref().unwrap();

                let destination_front_text = estimated_call
                    .destination_display.as_ref().unwrap()
                    .front_text.as_ref().unwrap();

                let aimed_departure_time = estimated_call
                    .aimed_departure_time;

                let Some(v) = format_departure_time(aimed_departure_time) else { continue };
                println!("v = {v}");
                
                let custom_format = match format_departure_time(aimed_departure_time) {
                    None => continue,
                    Some(value) => value,
                };

                println!("{:#?}", destination_front_text);

                let stop_place_data_row = StopPlaceDataRow {
                    transportMode: get_icon(transport_mode.as_str()),
                    publicCode: SharedString::from(public_code),
                    destinationFrontText: SharedString::from(destination_front_text),
                    aimedDepartureTime: SharedString::from(custom_format),
                };

                stop_place_data_rows.push(stop_place_data_row);
            }
            
            let stop_place_data = StopPlaceData {
                stopName: SharedString::from(format!("{} {}", quay.name, quay.public_code.unwrap_or(String::from("")))),
                stopDataRows: Rc::new(stop_place_data_rows).into()
            };

            let all_stops_data: VecModel<StopPlaceData> = VecModel::default();

            all_stops_data.push(stop_place_data);
            
            window.set_stopPlacesData(Rc::new(all_stops_data).into())
        })
        .unwrap();
}

pub async fn get_stop_place() -> Result<ResponseData, Box<dyn Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", header::HeaderValue::from_static("application/json"));
    headers.insert("ET-Client-Name",header::HeaderValue::from_static(ET_CLIENT));

    let variables = Variables {
        id: PRINSENS_GATE.to_string(),
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

pub fn format_departure_time(departure_time: DateTime) -> Option<String> {
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