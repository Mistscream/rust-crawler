extern crate chrono;  // for managing times
extern crate reqwest; // for http requests
extern crate select; // for parsing html documents

use self::chrono::prelude::*;
use self::select::document::Document;

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

    Some(ResponseData::new(String::from(url), body.unwrap()))
}

pub struct ResponseData {
    url: String,
    time: DateTime<Utc>,
    body: Document,
}

impl ResponseData {
    pub fn new(url: String, body: String) -> ResponseData {
        ResponseData {
            url: String::from(url),
            time: Utc::now(),
            body: Document::from(body.as_str()),
        }
    }

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

pub struct ResponseList {
    responses: Vec<ResponseData>,
}

impl ResponseList {
    pub fn new() -> ResponseList {
        ResponseList { responses: vec![] }
    }

    pub fn get_responses(&mut self) -> &mut Vec<ResponseData> {
        &mut self.responses
    }

    pub fn add_response(&mut self, data: ResponseData) {
        self.responses.push(data);
    }

    pub fn len(&self) -> usize {
        self.responses.len()
    }
}
