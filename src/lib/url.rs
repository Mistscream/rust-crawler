pub fn from_html(body: &str) -> Vec<String> {
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
        .collect::<Vec<String>>()
}
