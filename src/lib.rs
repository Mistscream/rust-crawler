extern crate chrono;
extern crate reqwest;
extern crate select;

use self::chrono::prelude::*;
use self::select::document::Document;
use self::select::predicate::Name;

//pub fn fat_request<'a>(url: &'a str) -> FatResponse<'a>   {
//    let res = get(url).unwrap();
//    let doc = Document::from_read(res).unwrap();
//    let links: Vec<&str> = doc.find(Name("a")).filter_map(|n| n.attr("href")).collect();
//
//    FatResponse {
//        url: url,
//        response: res,
//        document: doc,
//        links: links,
//    }
//}

#[derive(Debug)]
pub struct ResponseData {
    pub url: String, // make url &str ??
    pub time: DateTime<Utc>,
    pub body: Document,
}

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

pub fn find_links<'a>(body: &'a Document) -> Vec<&'a str> {
    body.find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .collect()
}
