use crate::calendar::calendar_models::{Calendar, CalendarEvent};
use std::fs;
use std::fs::File;
use std::io::Write;

pub async fn get_calendar() -> Calendar {
    let home_dir = dirs::home_dir();
    if (home_dir.is_none()) {
        return Calendar::default();
    }

    let json_path = home_dir.unwrap().join(".cal").join("calendar.json");

    let file_exists = json_path.try_exists();

    if (file_exists.is_err()) {
        // If we cannot check if the file exists, return an empty calendar
        return Calendar::default();
    }

    if (!file_exists.unwrap()) {
        // If the file does not exist, create an empty calendar
        save_calendar(&Calendar::default()).await;
        return Calendar::default();
    }

    let file = File::open(&json_path);
    if (file.is_err()) {
        return Calendar::default();
    }

    let json = file.unwrap();

    let calendar: Calendar = serde_json::from_reader(json).expect(&format!(
        "Failed to parse json file {}",
        json_path.display()
    ));

    calendar
}

pub async fn save_calendar(calendar: &Calendar) -> bool {
    let home_dir = dirs::home_dir();
    if (home_dir.is_none()) {
        return false;
    }

    let json_path = home_dir.unwrap().join(".cal").join("calendar.json");
    fs::create_dir_all(json_path.parent().unwrap()).expect("Failed to create directory");

    let calendar_json = serde_json::to_string(calendar).unwrap_or("".into());

    let result = fs::write(json_path, calendar_json);

    result.is_ok()
}

pub async fn add_event(calendar: Calendar, event: CalendarEvent) -> Calendar {
    let mut events = calendar.events;
    events.push(event);
    Calendar { events }
}
