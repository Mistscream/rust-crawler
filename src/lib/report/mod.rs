use chrono::{prelude::*, DateTime, Utc};
use lib::http::Response;

pub struct Report {
    url: String,
    requested_at: String,
    date: String,
    title: String,
    location: String,
    text: String,
}

impl Report {
    pub fn new(response: &Response) -> Report {
        let body = scraper::Html::parse_fragment(response.get_body());
        Report {
            url: String::from(response.get_url()),
            requested_at: response.get_time().to_rfc3339(),
            date: Report::parse_date(&body),
            title: Report::parse_title(&body),
            location: Report::parse_location(&body),
            text: String::from("text"),
        }
    }

    fn parse_date(body: &scraper::Html) -> String {
        let div = scraper::Selector::parse("div").unwrap();
        let date = body
            .select(&div)
            .filter(|h| h.value().attr("class").is_some())
            .filter(|h| h.value().attr("class").unwrap() == "polizeimeldung")
            .map(|elem| elem.inner_html())
            .nth(0);

        let date = match date {
            Some(d) => {
                let day = &d[19..21].parse::<u32>();
                let day = match day {
                    Ok(d) => *d,
                    Err(_) => 0,
                };
                let month = &d[22..24].parse::<u32>();
                let month = match month {
                    Ok(m) => *m,
                    Err(_) => 0,
                };
                let year = &d[25..29].parse::<i32>();
                let year = match year {
                    Ok(y) => *y,
                    Err(_) => 0,
                };
                Utc.ymd(year, month, day).and_hms(0, 0, 0)
            }
            None => Utc.ymd(0, 0, 0).and_hms(0, 0, 0),
        };

        date.to_rfc3339()
    }

    fn parse_title(body: &scraper::Html) -> String {
        let h1 = scraper::Selector::parse("h1").unwrap();
        let title = body
            .select(&h1)
            .filter(|h| h.value().attr("class").is_some())
            .filter(|h| h.value().attr("class").unwrap() == "title")
            .map(|elem| elem.inner_html())
            .nth(0);

        let title = match title {
            Some(t) => t,
            None => String::from("no title"),
        };

        title
    }

    fn parse_location(body: &scraper::Html) -> String {
        let div = scraper::Selector::parse("div").unwrap();
        let location = body
            .select(&div)
            .filter(|h| h.value().attr("class").is_some())
            .filter(|h| h.value().attr("class").unwrap() == "polizeimeldung")
            .map(|elem| elem.inner_html())
            .nth(1);

        let location = match location {
            Some(l) => l,
            None => String::from("no location"),
        };

        location
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

fn get_location(body: &str) -> String {
    let body = scraper::Html::parse_fragment(body);
    let div = scraper::Selector::parse("div").unwrap();

    let location = body
        .select(&div)
        .filter(|h| h.value().attr("class").is_some())
        .filter(|h| h.value().attr("class").unwrap() == "polizeimeldung")
        .map(|elem| elem.inner_html())
        .nth(1);

    match location {
        Some(l) => {
            println!("location: {}", l);
            l
        }
        None => String::from("no location"),
    }
}
