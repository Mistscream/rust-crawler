use chrono::{Date, DateTime, Utc};
use lib::http::Response;

pub struct Report {
    url: String,
    requested_at: DateTime<Utc>,
    date: DateTime<Utc>,
    id: u32,
    title: String,
    location: String,
    text: String,
}

impl Report {
    pub fn new(response: &Response) -> Report {
        Report {
            url: String::from(response.get_url()),
            requested_at: *response.get_time(),
            date: *response.get_time(),
            id: 0,
            title: String::from("title"),
            location: String::from("location"),
            text: String::from("text"),
        }
    }
}
