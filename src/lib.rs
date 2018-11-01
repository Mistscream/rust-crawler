extern crate chrono;
extern crate reqwest;
extern crate select;

use self::chrono::prelude::*;
use self::select::document::Document;
use self::select::predicate::Name;

/// Holds the information of a Response which is important for our crawler
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

pub fn process_links(body: &Document) -> Vec<String> {
    let links = find_links(body);
    let links = complete_links(links);
    let links = filter_relevant_links(links);

    links
}

/// Finds all links within a Document
fn find_links(body: &Document) -> Vec<String> {
    let vec: Vec<String> = body
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|s| String::from(s))
        .collect();

    vec
}

/// Completes all links, e.g. "/polizei/" -> "https://www.berlin.de/polizei/"
fn complete_links(links: Vec<String>) -> Vec<String> {
    let base_url = "https://www.berlin.de";
    let mut completed_links: Vec<String> = vec![];

    for link in links.iter() {
        if link.starts_with("/") {
            let mut url = String::from(base_url);
            url.push_str(&link[..]);
            completed_links.push(url);
        } else {
            completed_links.push(String::from(link.to_string()));
        }
    }

    completed_links
}

/// Removes all links not relevant for use case
fn filter_relevant_links(links: Vec<String>) -> Vec<String> {
    links
        .into_iter()
        .filter(|s| s.starts_with("https://www.berlin.de/polizei/polizeimeldungen"))
        .collect()
}
