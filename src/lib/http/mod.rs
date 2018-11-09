use chrono::{DateTime, Utc};
use regex::Regex;

pub fn get(url: &str) -> Option<Response> {
    println!("Sending get request to {}", &url);

    let response = reqwest::get(url);
    if response.is_err() {
        return None;
    }

    let mut response = response.unwrap();
    if !response.status().is_success() {
        return None;
    }

    let body = response.text();
    if body.is_err() {
        return None;
    }

    println!("Getting response from {}", &url);
    Some(Response::new(url, body.unwrap().as_str()))
}

pub struct Response {
    url: String,
    time: DateTime<Utc>,
    body: String,
    report: bool,
}

impl Response {
    pub fn new(url: &str, body: &str) -> Response {
        Response {
            url: String::from(url),
            time: Utc::now(),
            body: String::from(body),
            report: Response::check_for_report(&url),
        }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }

    pub fn get_body(&self) -> &str {
        &self.body
    }

    pub fn is_report(&self) -> bool {
        self.report
    }

    fn check_for_report(url: &str) -> bool {
        let ex = Regex::new(r"^(https?://www.berlin.de/polizei).*(pressemitteilung.[0-9]*.php)$")
            .unwrap();
        if ex.is_match(&url) {
            true
        } else {
            false
        }
    }
}
