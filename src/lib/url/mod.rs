extern crate select;

pub struct UrlList {
    urls: Vec<String>,
}

impl UrlList {
    pub fn new() -> UrlList {
        UrlList { urls: vec![] }
    }

    pub fn from_html(body: &str) -> UrlList {
        let base_url = "https://www.berlin.de";
        let body = select::document::Document::from(body);

        let mut urls: Vec<String> = body
            .find(select::predicate::Name("a"))
            .filter_map(|a| a.attr("href"))
            .map(|s| String::from(s))
            .map(|s| {
                if s.starts_with("/") {
                    format!("{}{}", base_url, s)
                } else {
                    s
                }
            }).filter(|s| s.starts_with("https://www.berlin.de/polizei/polizeimeldungen"))
            .collect();

        UrlList { urls: urls }
    }

    pub fn get_urls(&self) -> &Vec<String> {
        &self.urls
    }

    pub fn set_urls(&mut self, urls: Vec<String>) {
        self.urls = urls;
        self.remove_dups();
    }

    pub fn add_urls(&mut self, urls: &mut Vec<String>) {
        self.urls.append(urls);
        self.remove_dups();
    }

    pub fn add_url(&mut self, url: String) {
        self.urls.push(url);
        self.remove_dups();
    }

    pub fn add_url_list(&mut self, list: &mut UrlList) {
        self.urls.append(&mut list.urls);
        self.remove_dups();
    }

    pub fn remove_urls(&mut self, urls: &UrlList) {
        self.urls = self
            .get_urls()
            .into_iter()
            .filter(|u| !urls.get_urls().contains(u))
            .map(|s| s.to_string())
            .collect();
    }

    fn remove_dups(&mut self) {
        self.urls.sort_unstable();
        self.urls.dedup();
    }

    pub fn len(&self) -> usize {
        self.urls.len()
    }
}
