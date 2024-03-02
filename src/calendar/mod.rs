extern crate ical;

use crate::ui::*;


use chrono::{ DateTime, Local, Locale, NaiveDateTime, TimeZone};
use ical::parser::ical::component::IcalEvent;
use log::{debug, error};
use slint::{VecModel, Weak};
use std::{rc::Rc, thread};

const CAL_URL: &str ="https://outlook.office365.com/owa/calendar/4ccddc187e214383b2914f75061813b6@knowit.no/06d7c824a8c0401a9d0519fbccb7d29d8889045414010058404/calendar.ics";

#[derive(Debug)]
pub struct CalendarEvent {
    pub summary: String,
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
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
            let calendar_events: VecModel<Event> = VecModel::default();

            let mut upcoming_events: Vec<CalendarEvent> = calendar.into_iter().filter(|x| x.end_time >= Local::now()).collect();
            upcoming_events.sort_by(|a,b| a.start_time.cmp(&b.start_time));

            for event in &upcoming_events[0..3] {
                let date_and_start_time = event.start_time.format_localized("%-d %B %H:%M",Locale::nb_NO);
                let end_time = event.end_time.format_localized("%H:%M", Locale::nb_NO);
                let summary = &event.summary;
                calendar_events.push( Event {
                    summary: summary.into(),
                    date: format!("{0}-{1}",date_and_start_time,end_time).into()
                });
            }
            
            window.set_events(Rc::new(calendar_events).into());
        })
        .unwrap();
}

pub async fn get_current_calendar() -> Vec<CalendarEvent> {

    let result = fetch_calendar().await;

    let events = if let Ok(result) = result {
        parse_calendar(result.as_bytes()).await
    } else {
        vec!()
    };

    debug!("Loaded events from calendar: {:?}", events);

    events
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
        let mut start_time:Option<DateTime<Local>> = None;
        let mut end_time:Option<DateTime<Local>> = None;
        for property in event.properties {
            let name: &str = property.name.as_str();
            let value: String = property.value.unwrap_or(String::from(""));

            if value.is_empty() {continue;}

            match name {
               "SUMMARY" => summary = Some(value),
               "DTSTART" => start_time = parse_date(value).await,
               "DTEND" => end_time = parse_date(value).await,
                _ => ()
            }
        }

        if summary.is_some() && start_time.is_some() && end_time.is_some() {
            calender_events.push(CalendarEvent {summary: summary.unwrap(), start_time: start_time.unwrap(), end_time: end_time.unwrap()})
        }
    }

    calender_events
}

pub async fn parse_date(date_string: String) -> Option<DateTime<Local>> {
    
    let date = match NaiveDateTime::parse_from_str(&date_string, "%Y%m%dT%H%M%S") {
        Ok(datetime) => Some(datetime),
        Err(err) => { 
            println!("Error parsing datetime: {:?}", err);
            None
        }
    };

    let local_date = match date {
        Some(date) => Local.from_local_datetime(&date),
        None => todo!(),
    };

    match local_date {
        chrono::LocalResult::None => None,
        chrono::LocalResult::Single(a) => Some(a),
        chrono::LocalResult::Ambiguous(a, _) => Some(a),
    }

}