use chrono::{Date, DateTime, Utc};
use lib::http::Response;


pub struct Report {
    url: String,
    requested_at: DateTime<Utc>,
    date: Date<Utc>,
    id: u32,
    title: String,
    location: String,
    text: String,
}

impl Report {
    pub fn new(response: &Response) {

    }
}