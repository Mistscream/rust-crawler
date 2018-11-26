use select::document::Document;
use select::predicate::*;

pub fn from_html(html: &str) -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();
    let document = Document::from(html);

    for elem in document.find(Name("a")) {
        match elem.attr("href") {
            Some(url) => {
                let mut url = String::from(url);
                if url.starts_with("/") {
                    url.insert_str(0, "https://www.berlin.de");
                }
                if url.starts_with("https://www.berlin.de/polizei/polizeimeldungen/archiv/20")
                    && url.contains("page_at")
                {
                    urls.push(url);
                }
            }
            None => (),
        }
    }

    urls
}
