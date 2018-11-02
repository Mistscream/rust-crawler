extern crate select;

use self::select::document::Document;
use self::select::predicate::Name;

pub fn process(body: &Document) -> Vec<String> {
    let links = find(body);
    let links = complete(links);
    let links = filter_relevant(links);

    links
}

fn find(body: &Document) -> Vec<String> {
    let vec: Vec<String> = body
        .find(Name("a"))
        .filter_map(|a| a.attr("href"))
        .map(|s| String::from(s))
        .collect();

    vec
}

fn complete(links: Vec<String>) -> Vec<String> {
    let base_url = "https://www.berlin.de";
    let mut completed_links: Vec<String> = vec![];

    for link in links.iter() {
        if link.starts_with("/") {
            let mut url = String::from(base_url);
            url.push_str(link);
            completed_links.push(url);
        } else {
            completed_links.push(String::from(link.to_string()));
        }
    }

    completed_links
}

fn filter_relevant(links: Vec<String>) -> Vec<String> {
    links
        .into_iter()
        .filter(|s| s.starts_with("https://www.berlin.de/polizei/polizeimeldungen"))
        .collect()
}