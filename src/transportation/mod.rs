use reqwest;
use reqwest::header;
use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use std::rc::Rc;
use std::thread;
use chrono::DateTime as ChronoDateTime;
use slint::{ComponentHandle, SharedString, VecModel, Weak};
use crate::transportation::stop_place::{ResponseData, StopPlaceStopPlace};
use crate::ui::{MainWindow, StopPlaceData, StopPlaceDataRow};

const BASE_URL: &str = "https://api.entur.io/journey-planner/v3/graphql";
// https://developer.entur.org/pages-intro-authentication
const ET_CLIENT: &str = "knowit-objectnet-trd-infoscreen";
const HAAKON_VII_GATE: &str = "NSR:StopPlace:42310";
const DEFAULT_TIME_RANGE: i64 = 72100;
const DEFAULT_NUMBER_OF_DEPARTURES: i64 = 10;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/resources/journey-planner_schema.json",
    query_path = "src/resources/journey-planner_query.graphql",
    response_derives = "Debug"
)]
struct StopPlace;


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

fn display_transportation(window_weak: &Weak<MainWindow>, stop_place: StopPlaceStopPlace) {
    
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {

            let transport_mode = stop_place
                .estimated_calls.get(0).unwrap()
                .service_journey.journey_pattern.as_ref().unwrap()
                .line.transport_mode.as_ref().unwrap();

            let public_code = stop_place
                .estimated_calls.get(0).unwrap()
                .service_journey.journey_pattern.as_ref().unwrap()
                .line.public_code.as_ref().unwrap();
            
            let destination_front_text = stop_place
                .estimated_calls.get(0).unwrap()
                .destination_display.as_ref().unwrap()
                .front_text.as_ref().unwrap();
            
            let aimed_departure_time = &stop_place
                .estimated_calls.get(0).unwrap()
                .aimed_departure_time;

            let custom_format = aimed_departure_time.as_str();
            //let custom_format_2 = aimed_departure_time.fmt();
            
            println!("{:#?}", destination_front_text);
            
            let stop_place_data_rows: VecModel<StopPlaceDataRow> = VecModel::default();

            let stop_place_data_row = StopPlaceDataRow {
                transportMode: SharedString::from("bus"),
                publicCode: SharedString::from(public_code),
                destinationFrontText: SharedString::from(destination_front_text),
                aimedDepartureTime: SharedString::from(custom_format),
            };
            
            stop_place_data_rows.push(stop_place_data_row);
            
            let stop_place_data = StopPlaceData {
                stopName: SharedString::from(stop_place.name),
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

    let variables = stop_place::Variables {
        id: HAAKON_VII_GATE.to_string(),
        time_range: Some(DEFAULT_TIME_RANGE),
        number_of_departures: Some(DEFAULT_NUMBER_OF_DEPARTURES),
    };
    
    let request_body = StopPlace::build_query(variables);
    
    let client = reqwest::Client::new();

    let res = client.post(BASE_URL)
        .headers(headers)
        .json(&request_body)
        .send().await?;
    
    let response_body: Response<ResponseData> = res.json().await?;

    println!("{:#?}", response_body);

    Ok(response_body.data.expect("Missing response data"))
}
