pub struct Url {
    url: String,
    visited: bool,
}

impl Url {
    pub fn new(url: &str) -> Url {
        Url {
            url: String::from(url),
            visited: false,
        }
    }

    pub fn is_visited(&self) -> bool {
        self.visited
    }

    pub fn set_visited(&mut self, status: bool) {
        self.visited = status;
    }

    pub fn as_str(&self) -> &str {
        &self.url
    }
}

impl Ord for Url {
    fn cmp(&self, other: &Url) -> std::cmp::Ordering {
        self.url.cmp(&other.url)
    }
}

impl PartialEq for Url {
    fn eq(&self, other: &Url) -> bool {
        self.url == other.url
    }
}

impl PartialOrd for Url {
    fn partial_cmp(&self, other: &Url) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Url {}

pub fn from_html(body: &str) -> Vec<Url> {
    let base_url = "https://www.berlin.de";
    let body = scraper::Html::parse_fragment(body);
    let anker_tags = scraper::Selector::parse("a").unwrap();

    body.select(&anker_tags)
        .filter_map(|a| a.value().attr("href"))
        .map(|s| String::from(s))
        .map(|s| {
            if s.starts_with("/") {
                format!("{}{}", base_url, s)
            } else {
                s
            }
        }).filter(|s| s.starts_with("https://www.berlin.de/polizei/polizeimeldungen/archiv/20"))
        .map(|s| Url::new(&s))
        .collect::<Vec<Url>>()
}
