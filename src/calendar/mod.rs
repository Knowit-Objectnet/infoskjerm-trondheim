extern crate ical;

use crate::ui::*;


use chrono::{DateTime, Local, NaiveDateTime, Offset};
use ical::parser::ical::component::IcalEvent;
use log::{error, info};
use slint::{VecModel, Weak};
use tide::new;
use std::{error, fmt, fs::File, io::BufReader, rc::Rc, thread};


const CAL_URL: &str ="https://outlook.office365.com/owa/calendar/4ccddc187e214383b2914f75061813b6@knowit.no/06d7c824a8c0401a9d0519fbccb7d29d8889045414010058404/calendar.ics";

#[derive(Debug)]
pub struct CalendarEvent {
    pub summary: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
}

pub fn setup(window: &MainWindow) {
    let window_weak = window.as_weak();
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(calendar_worker_loop(window_weak))
    });
}

async fn calendar_worker_loop(window: Weak<MainWindow>) {
    loop {
        let current_calendar = get_current_calendar().await;
        display_calendar(&window, current_calendar).await;
        tokio::time::sleep(std::time::Duration::from_secs(60 * 15)).await;
    }
}


async fn display_calendar(window_weak: &Weak<MainWindow>, calendar: Vec<CalendarEvent>) {
    // slint::Image is not thread safe (Send), see https://slint.dev/releases/1.1.0/docs/rust/slint/struct.Image#sending-image-to-a-thread
    

    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            let calendar_events: VecModel<Cal> = VecModel::default();

            for event in calendar {
                let event_string: String = format!("{} - {}-{}",event.summary,event.start_time.format("%Y-%m-%d %H:%M").to_string(),event.end_time.format("%H:%M").to_string());

                calendar_events.push( Cal {
                    summary: event_string.into(),
                });
            }
            


            window.set_events(Rc::new(calendar_events).into());
        })
        .unwrap();
}

pub async fn get_current_calendar() -> Vec<CalendarEvent> {

    let foo = fetch_calendar().await;

    let x = if let Ok(foo) = foo {
        parse_calendar(foo.as_bytes()).await
    } else {
        vec!()
    };

    info!("Loaded events: {:?}", x);

    x
}

pub async fn fetch_calendar() -> Result<String, reqwest::Error> {

    let response = reqwest::get(CAL_URL).await?;

    response.text().await
    
}

pub async fn parse_calendar(cal_to_parse: &[u8]) -> Vec<CalendarEvent> {

    let reader = ical::IcalParser::new(cal_to_parse);

    let mut events_unparsed: Vec<IcalEvent> = vec!();

    for line in reader {
       match line {
            Ok(line) => for event in line.events { events_unparsed.push(event)},
            Err(err) => error!("Error while trying to parse calender events response: {}",err),
        };
    }

    let mut calender_events: Vec<CalendarEvent> = vec!();

    for event in events_unparsed {
        let mut summary:Option<String> = None;
        let mut startTime:Option<NaiveDateTime> = None;
        let mut endTime:Option<NaiveDateTime> = None;
        for property in event.properties {
            let name: &str = property.name.as_str();
            let value: String = property.value.unwrap_or(String::from(""));

            if value.is_empty() {continue;}

            match name {
               "SUMMARY" => summary = Some(value),
               "DTSTART" => startTime = parse_date(value).await,
               "DTEND" => endTime = parse_date(value).await,
                _ => ()
            }
        }

        if summary.is_some() && startTime.is_some() && endTime.is_some() {
            calender_events.push(CalendarEvent {summary: summary.unwrap(), start_time: startTime.unwrap(), end_time: endTime.unwrap()})
        }
    }

    calender_events
}

pub async fn parse_date(date_string: String) -> Option<NaiveDateTime> {
    match NaiveDateTime::parse_from_str(&date_string, "%Y%m%dT%H%M%S") {
        Ok(datetime) => Some(datetime),
        Err(err) => { 
            println!("Error parsing datetime: {:?}", err);
            None
        }
    }
}