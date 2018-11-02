extern crate chrono;
extern crate reqwest;
extern crate select;

use self::chrono::prelude::*;
use self::select::document::Document;

pub struct ResponseData {
    url: String,
    time: DateTime<Utc>,
    body: Document,
}

impl ResponseData {
    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }

    pub fn get_body(&self) -> &Document {
        &self.body
    }
}

pub fn get(url: &str) -> Option<ResponseData> {
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

    let body = body.unwrap();

    Some(ResponseData {
        url: String::from(url),
        time: Utc::now(),
        body: Document::from(body.as_str()),
    })
}
