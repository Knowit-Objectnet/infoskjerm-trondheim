use askama::Template;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Template, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[template(
    ext = "html",
    source = "<tr>
        <td>{{ summary }}</td>
        <td>{{ start_time.format(\"%Y-%m-%d %H:%M\") }}</td>
        <td>{{ stop_time.format(\"%Y-%m-%d %H:%M\") }}</td>
        <td><button class=\"delete-event\" data-id=\"{{ id }}\" onclick=\"deleteEvent(event)\">Delete</button></td>\
        </tr>"
)]
#[serde(rename_all = "camelCase")]
pub struct CalendarEvent {
    pub id: Uuid,
    pub summary: String,
    pub start_time: DateTime<Local>,
    pub stop_time: DateTime<Local>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalCalendarEvent {
    pub summary: String,
    pub start_time: DateTime<Local>,
    pub stop_time: DateTime<Local>,
}

#[derive(Template, Default, Debug, PartialEq, Serialize, Deserialize)]
#[template(
    ext = "html",
    source = r#"<table id=\"calendar-list\">
    <caption>Existing Events</caption>
    <thead>

        <tr>
            <th style="width: 300px">Summary</th>
            <th style="width: 150px">Start Time</th>
            <th style="width: 150px">Stop Time</th>
            <th style="width: 50px">Actions</th>
        </tr>
        </thead>
        <tbody>
    {% for event in events %}
        {{ event | safe }}
        {% endfor %}
        </tbody>
        </table>"#
)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    pub events: Vec<CalendarEvent>,
}
