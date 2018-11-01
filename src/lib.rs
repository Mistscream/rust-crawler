extern crate chrono;
extern crate reqwest;
extern crate select;

use self::chrono::prelude::*;
use self::select::document::Document;
use self::select::predicate::Name;


/// Holds the information of a Response which is important for our crawler
#[derive(Debug)]
pub struct ResponseData {
    pub url: String, // make url &str ??
    pub time: DateTime<Utc>,
    pub body: Document,
}

/// Starts a get request to given url and returns ResponseData
pub fn request(url: &str) -> Option<ResponseData> {
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

/// Finds all links within a Document
pub fn find_links<'a>(body: &'a Document) -> Vec<String> {
    body.find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|s| String::from(s))
        .collect()
}
