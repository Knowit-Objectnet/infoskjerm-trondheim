use std::collections::HashMap;
use ::reqwest::blocking::Client;
use reqwest::header;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use slint::VecModel;

const BASE_URL: &str = "https://api.entur.io/journey-planner/v3/graphql";
// https://developer.entur.org/pages-intro-authentication
const ET_CLIENT: &str = "knowit-objectnet-trd-infoscreen";
const HAAKON_VII_GATE: &str = "NSR:StopPlace:42310";
const DEFAULT_TIME_RANGE: i32 = 72100;
const DEFAULT_NUMBER_OF_DEPARTURES: i32 = 10;

type Date = String;
type DateTime = String;

#[derive(GraphQLQuery)]
#[graphql(
schema_path = "src/resources/journey-planner_schema.json",
query_path = "src/resources/journey-planner_query.graphql",
response_derives = "Debug"
)]
struct StopPlace;

pub fn test_graph_ql() -> Result<(), Box<dyn std::error::Error>> {

    let mut headers = header::HeaderMap::new();
    headers.insert("Accept", header::HeaderValue::from_static("application/json"));
    headers.insert("ET-Client-Name",header::HeaderValue::from_static(ET_CLIENT));

    // get a client builder
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    let variables = stop_place::Variables {
        id: HAAKON_VII_GATE,
        timeRange: DEFAULT_TIME_RANGE,
        numberOfDepartures: DEFAULT_NUMBER_OF_DEPARTURES,
    };

    let response_body =
        post_graphql::<StopPlace, _>(&client, BASE_URL, variables).unwrap();

}