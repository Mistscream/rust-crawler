use chrono::{DateTime, Utc};

pub fn get(url: &str) -> Option<Response> {
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

    Some(Response::new(url, body.unwrap().as_str()))
}

pub struct Response {
    url: String,
    time: DateTime<Utc>,
    body: String,
}

impl Response {
    pub fn new(url: &str, body: &str) -> Response {
        Response {
            url: String::from(url),
            time: Utc::now(),
            body: String::from(body),
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
}
