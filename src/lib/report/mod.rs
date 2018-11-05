use lib::http::Response;
use chrono::{Date, DateTime, Utc};


// pub fn extract(body: &str) -> Option<Report> {
// }

pub struct Report {
    url: String,
    requested_at: DateTime<Utc>,
    date: Date<Utc>,
    id: u32,
    title: String,
    location: String,
    text: String,
}