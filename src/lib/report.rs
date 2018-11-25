use chrono::prelude::*;
use lib::*;
use select::document::Document;
use select::predicate::*;

pub struct Report {
    url: String,
    requested_at: String,
    date: String,
    title: String,
    location: String,
    text: String,
}

impl Report {
    pub fn new(body: &str) -> Report {
        Report {
            url: "bla".to_string(),
            requested_at: "bla".to_string(),
            date: "bla".to_string(),
            title: "bla".to_string(),
            location: "bla".to_string(),
            text: String::from("text"),
        }
    }
}

#[test]
fn test1() {
    let body =
        crate::lib::request::get("https://www.berlin.de/polizei/polizeimeldungen/archiv/2014");
    // crate::lib::request::get_request("https://www.berlin.de/polizei/polizeimeldungen/archiv/2018");
    crate::lib::report::from_html(&body.unwrap());
}
pub fn from_html(html: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();

    let document = Document::from(html);
    let li_elements: Vec<select::node::Node> = Vec::new();
    for elem in document.find(Class("list-autoteaser").descendant(Class("row-fluid"))) {
        let title = parse_title(elem);
        let url = parse_url(elem);
        let location = parse_location(elem);
        let date = parse_date(elem);
        let text = parse_text(&url);

        println!("{}\n{}\n{}\n{}\n{}", title, url, location, date, text);
    }

    reports
}

fn parse_title(elem: select::node::Node) -> String {
    match elem.find(Name("a")).next() {
        Some(a) => String::from(a.text()),
        None => String::from("no title"),
    }
}

fn parse_url(elem: select::node::Node) -> String {
    let url = match elem.find(Name("a")).next() {
        Some(a) => match a.attr("href") {
            Some(href) => String::from(href),
            None => String::from("no url"),
        },
        None => String::from("no url"),
    };

    if url.starts_with("/") {
        format!("{}{}", "https://www.berlin.de", url)
    } else {
        url
    }
}

fn parse_location(elem: select::node::Node) -> String {
    match elem.find(Class("category")).next() {
        Some(l) => String::from(l.text()),
        None => String::from("no location"),
    }
}

fn parse_date(elem: select::node::Node) -> String {
    match elem.find(Class("span2")).next() {
        Some(date) => {
            let date = date.text();

            let day = match &date[0..2].parse::<u32>() {
                Ok(d) => *d,
                Err(_) => 1,
            };

            let month = match &date[3..5].parse::<u32>() {
                Ok(m) => *m,
                Err(_) => 1,
            };

            let year = match &date[6..10].parse::<i32>() {
                Ok(y) => *y,
                Err(_) => 1970,
            };

            let hour = match &date[11..13].parse::<u32>() {
                Ok(h) => *h,
                Err(_) => 12,
            };

            let minute = match &date[14..16].parse::<u32>() {
                Ok(m) => *m,
                Err(_) => 0,
            };

            FixedOffset::east(1 * 3600)
                .ymd(year, month, day)
                .and_hms(hour, minute, 0)
                .to_rfc3339()
        }
        None => FixedOffset::east(1 * 3600)
            .ymd(1970, 1, 1)
            .and_hms(0, 0, 0)
            .to_rfc3339(), // let body = lib::request::get(&url);
    }
}

fn parse_text(url: &str) -> String {
    let html = request::get(&url);
    if !html.is_some() {
        return String::from("no text");
    }

    let html = html.unwrap();
    let document = Document::from(html.as_str());
    let mut text: String = String::new();
    for elem in document.find(Class("span7").descendant(Class("textile"))) {
        for p in elem.find(Name("p")) {
            println!("{}", p.text());
            text.push_str(&p.text());
        }
    }

    if text.len() == 0 {
        String::from("no text")
    } else {
        text
    }
}
