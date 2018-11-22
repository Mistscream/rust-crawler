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
            title: get_title(response.get_body()),
            location: String::from("location"),
            text: String::from("text"),
        }
    }
}

fn get_title(body: &str) -> String {
    let body = scraper::Html::parse_fragment(body);
    let h1 = scraper::Selector::parse("h1").unwrap();

    let title = body
        .select(&h1)
        .filter(|h| h.value().attr("class").is_some())
        .filter(|h| h.value().attr("class").unwrap() == "title")
        .map(|elem| elem.inner_html())
        .nth(0);

    match title {
        Some(t) => {
            println!("title: {}", t);
            t
        }
        None => String::from("no title"),
    }
}
