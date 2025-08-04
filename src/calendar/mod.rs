mod calendar_models;
mod server;

mod storage;

use crate::ui::*;

use crate::calendar::calendar_models::CalendarEvent;
use crate::calendar::storage::get_calendar;
use chrono::{Local, Locale, TimeZone};
use slint::{VecModel, Weak};
use std::{cmp::min, env, rc::Rc, thread};
use tokio::runtime::Runtime;

fn get_server_url() -> String {
    env::var("CALENDAR_SERVER_URL").unwrap_or(String::from("http://localhost:1338/"))
}

pub fn setup(window: &MainWindow) {
    let window_weak = window.as_weak();

    //spawn server worker thread
    thread::spawn(move || {
        Runtime::new()
            .unwrap()
            .block_on(server::calendar_endpoint_server())
    });
    //thread for displaying calendar events
    thread::spawn(move || {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(calendar_worker_loop(window_weak))
    });
}

async fn calendar_worker_loop(window: Weak<MainWindow>) {
    loop {
        let current_calendar = get_calendar().await;
        display_calendar(&window, current_calendar.events).await;
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}

async fn display_calendar(window_weak: &Weak<MainWindow>, calendar: Vec<CalendarEvent>) {
    window_weak
        .upgrade_in_event_loop(move |window: MainWindow| {
            let calendar_events: VecModel<Event> = VecModel::default();

            let mut upcoming_events: Vec<CalendarEvent> = calendar
                .into_iter()
                .filter(|x| x.stop_time >= Local::now())
                .collect();
            upcoming_events.sort_by(|a, b| a.start_time.cmp(&b.start_time));

            let take_count = min(3, upcoming_events.len());

            for event in &upcoming_events[0..take_count] {
                let date_and_start_time = event
                    .start_time
                    .format_localized("%-d %B %H:%M", Locale::nb_NO);
                let end_time = event.stop_time.format_localized("%H:%M", Locale::nb_NO);
                let summary = &event.summary;
                calendar_events.push(Event {
                    summary: summary.into(),
                    date: format!("{0}-{1}", date_and_start_time, end_time).into(),
                });
            }

            window.set_events(Rc::new(calendar_events).into());

            let mut server_url = get_server_url();
            if !server_url.ends_with('/') {
                server_url.push('/');
            }
            window.set_calendarServerUrl(server_url.into());
        })
        .unwrap();
}
