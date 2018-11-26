use chrono::prelude::*;
use lib::*;
use select::document::Document;
use select::predicate::*;

pub struct Report {
    title: String,
    url: String,
    location: String,
    date: String,
    text: String,
}

impl Report {
    pub fn new(title: String, url: String, location: String, date: String, text: String) -> Report {
        Report {
            title: title,
            url: url,
            location: location,
            date: date,
            text: text,
        }
    }

    pub fn get_fields(&self) -> Vec<&str> {
        vec![
            &self.url,
            &self.title,
            &self.date,
            &self.location,
            &self.text,
        ]
    }

    pub fn from_html(html: &str) -> Vec<Report> {
        let mut reports: Vec<Report> = Vec::new();

        let document = Document::from(html);
        let li_elements: Vec<select::node::Node> = Vec::new();
        for elem in document.find(Class("list-autoteaser").descendant(Class("row-fluid"))) {
            println!("found report: parsing data");
            let title = Report::parse_title(elem);
            let url = Report::parse_url(elem);
            let location = Report::parse_location(elem);
            let date = Report::parse_date(elem);
            let text = Report::parse_text(&url);

            if text.len() > 0 {
                reports.push(Report::new(title, url, location, date, text));
            }
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
            Some(l) => String::from(&l.text()[13..]),
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
            return String::from("");
        }

        let html = html.unwrap();
        let document = Document::from(html.as_str());
        let mut text: String = String::new();
        for elem in document.find(Class("span7").descendant(Class("textile"))) {
            for p in elem.find(Name("p")) {
                text.push_str(&p.text());
            }
        }

        text
    }
}
