use chrono::prelude::*;
use lib::*;

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
        let body = scraper::Html::parse_fragment(body);
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
pub fn from_html(body: &str) -> Vec<Report> {
    // vector which will store all reports and be returned from function
    let mut reports: Vec<Report> = Vec::new();

    // parse body and declare selectors
    let body = scraper::Html::parse_fragment(body);
    let sel_ul = scraper::Selector::parse("ul").unwrap();
    let sel_li = scraper::Selector::parse("li").unwrap();
    let sel_div = scraper::Selector::parse("div").unwrap();

    // find ul-element which has reports as li-elements
    let ul = body
        .select(&sel_ul)
        // .filter_map(|ul| ul.value().attr("class"))
        // .filter(|ul| ul == "list")
        .filter(|ul| ul.value().attr("class").is_some())
        .filter(|ul| ul.value().attr("class").unwrap() == "list-autoteaser")
        .map(|ul| ul.inner_html())
        .map(|ul| scraper::Html::parse_fragment(&ul))
        .nth(0);

    // return when there is no ul-element which has reports
    if !ul.is_some() {
        return reports;
    }

    // store each li-element as independent html fragment
    let list_items: Vec<scraper::Html> = ul
        .unwrap()
        .select(&sel_li)
        .map(|li| li.inner_html())
        .map(|li| scraper::Html::parse_fragment(&li))
        .collect();

    // create report from each li-element
    for li in list_items {
        let date = get_date(&li);
        println!("{}", date);
        let title = get_title(&li);
        println!("{}", title);
        let location = get_location(&li);
        let mut url = get_url(&li);
        url = format!("{}{}", "https://www.berlin.de", url);
        println!("{}", url);
        let text = get_text(&url);
        println!("{}\n\n", text);
    }

    reports
}

fn get_date(html: &scraper::Html) -> String {
    let sel_div = scraper::Selector::parse("div").unwrap();
    let date: String = html
        .select(&sel_div)
        .filter(|div| div.value().attr("class").is_some())
        .filter(|div| div.value().attr("class").unwrap() == "span2 cell date")
        .map(|div| div.inner_html())
        .collect();

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

fn get_title(html: &scraper::Html) -> String {
    let sel_a = scraper::Selector::parse("a").unwrap();
    html.select(&sel_a).map(|a| a.inner_html()).collect()
}

fn get_location(html: &scraper::Html) -> String {
    String::from("bla")
}

fn get_url(html: &scraper::Html) -> String {
    let sel_a = scraper::Selector::parse("a").unwrap();
    html.select(&sel_a)
        .filter(|a| a.value().attr("href").is_some())
        .map(|a| a.value().attr("href").unwrap())
        .collect()
}

fn get_text(url: &str) -> String {
    let body = request::get(&url);
    if !body.is_some() {
        return String::from("empty");
    }

    let body = scraper::Html::parse_fragment(&body.unwrap());
    let sel_div = scraper::Selector::parse("div").unwrap();
    let sel_p = scraper::Selector::parse("p").unwrap();

    let story_div = body
        .select(&sel_div)
        .filter(|div| div.value().attr("class").is_some())
        .filter(|div| div.value().attr("class").unwrap() == "span7 column-content")
        // .filter(|d| d == &"span7 column-content")
        .map(|div| div.inner_html())
        .map(|div| scraper::Html::parse_fragment(&div))
        .nth(0);

    if !story_div.is_some() {
        return String::from("empty");
    }

    let p = story_div
        .unwrap()
        .select(&sel_p)
        .map(|p| p.inner_html())
        .nth(0);
    
    if !p.is_some() {
        return String::from("empty");
    }

    p.unwrap()
}
