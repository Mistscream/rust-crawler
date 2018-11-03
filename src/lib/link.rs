extern crate select;

use self::select::document::Document;
use self::select::predicate::Name;

pub struct UrlQueue {
    urls: Vec<String>,
}

impl UrlQueue {
    pub fn new() -> UrlQueue {
        UrlQueue { urls: vec![] }
    }

    pub fn get_urls(&self) -> &Vec<String> {
        &self.urls
    }

    fn set_urls(&mut self, urls: Vec<String>) {
        self.urls = urls;
    }

    pub fn len(&self) -> usize {
        self.urls.len()
    }

    pub fn add_vec(&mut self, new_links: &mut Vec<String>) {
        self.urls.append(new_links);
        self.urls.sort_unstable();
        self.urls.dedup();
    }

    pub fn add(&mut self, url: String) {
        self.urls.push(url);
        self.urls.sort_unstable();
        self.urls.dedup();
    }

    pub fn remove_from(&mut self, other: &Self) {
        let urls: Vec<String> = self
            .get_urls()
            .into_iter()
            .filter(|u| !other.get_urls().contains(u))
            .map(|s| s.to_string())
            .collect();
        
        self.set_urls(urls);
    }
}

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

    links
        .into_iter()
        .map(|s| {
            if s.starts_with("/") {
                format!("{}{}", base_url, s)
            } else {
                s
            }
        }).collect()
}

fn filter_relevant(links: Vec<String>) -> Vec<String> {
    links
        .into_iter()
        .filter(|s| s.starts_with("https://www.berlin.de/polizei/polizeimeldungen"))
        .collect()
}
