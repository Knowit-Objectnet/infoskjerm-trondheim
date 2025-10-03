use crate::calendar::calendar_models::{Calendar, CalendarEvent, ExternalCalendarEvent};
use crate::calendar::storage::{add_event, get_calendar, save_calendar};
use askama::Template;
use chrono::Local;
use log::{error, info};
use reqwest::Url;
use serde::Deserialize;
use std::env;
use tide::{Redirect, Request};
use uuid::Uuid;

#[derive(Template)] // this will generate the code...
#[template(path = "calendar.html")] // using the template in this path, relative
// to the `templates` dir in the crate root
struct CalendarTemplate<'a> {
    // the name of the struct can be anything
    calendar: &'a Calendar, // the field name should match the variable name
                            // in your template
}

fn get_server_url() -> String {
    env::var("CALENDAR_SERVER_URL").unwrap_or_else(|_| String::from("http://localhost:1338/"))
}

pub async fn calendar_endpoint_server() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/calendar").get(move |_| async move {
        // Load the calendar data
        let calendar = get_calendar().await;
        // Render the template with the calendar data
        let template = CalendarTemplate {
            calendar: &calendar,
        };
        let response = tide::Response::builder(200)
            .content_type(tide::http::mime::HTML)
            .body(template.render().unwrap())
            .build();
        Ok(response)
    });

    app.at("/calendar/event")
        .post(move |req| add_event_by_req(req))
        .delete(move |req| delete_event_by_req(req));

    let server_url = get_server_url();
    app.listen(server_url).await?;
    Ok(())
}

async fn add_event_by_req(mut req: Request<()>) -> tide::Result {
    let result_body = req.body_json::<ExternalCalendarEvent>().await;

    if (result_body.is_err()) {
        error!(
            "Failed to parse event data: {:?}",
            result_body.err().unwrap().type_name()
        );

        return Err(tide::Error::from_str(
            tide::StatusCode::BadRequest,
            "Failed to parse event data",
        ));
    }

    let event: ExternalCalendarEvent = result_body.unwrap();

    let event_with_id = CalendarEvent {
        id: Uuid::new_v4(),
        summary: event.summary,
        start_time: event.start_time,
        stop_time: event.stop_time,
    };

    let mut calendar = get_calendar().await;
    calendar = add_event(calendar, event_with_id).await;
    // Save the updated calendar
    save_calendar(&calendar).await;
    Ok("Event added successfully".to_string().into())
}

async fn delete_event_by_req(mut req: Request<()>) -> tide::Result {
    let event_id: String = req.body_json().await?;
    let mut calendar = get_calendar().await;
    calendar
        .events
        .retain(|event| event.id.to_string() != event_id);
    // Save the updated calendar
    save_calendar(&calendar).await;
    Ok("Deleted".to_string().into())
}

#[derive(Debug, Deserialize)]
struct Tracking {
    url: Url,
}
