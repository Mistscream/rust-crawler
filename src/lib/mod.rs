mod http;
mod report;
mod url;

pub fn run(start_url: &str) {
    let mut url_queue = url::UrlList::new();
    let mut visited_urls = url::UrlList::new();
    let mut responses: Vec<http::Response> = vec![];

    url_queue.add_url(start_url.to_string());

    loop {
        // make requests to all urls in queue
        println!("{} urls in queue: starting requests", url_queue.len());
        make_requests(&mut url_queue, &mut visited_urls, &mut responses);

        // extracting links from response data
        println!("Got {} responses: starting link search", responses.len());
        extract_links(&mut url_queue, &mut responses);

        // removing urls from queue which have recently been requested
        println!("{} urls in queue: removing recently used", url_queue.len());
        url_queue.remove_urls(&visited_urls);

        // stop crawling when there are no new unvisited urls
        if url_queue.len() == 0 {
            std::process::exit(0);
        }
    }
}

fn make_requests(
    url_q: &mut url::UrlList,
    visited_q: &mut url::UrlList,
    responses: &mut Vec<http::Response>,
) {
    for url in url_q.get_urls().iter() {
        println!("Sending request to {}", url);
        let response = http::get(url);
        if response.is_some() {
            responses.push(response.unwrap());
            println!("Request successful");
        }
        visited_q.add_url(url.to_string());
    }
}

fn extract_links(url_q: &mut url::UrlList, responses: &mut Vec<http::Response>) {
    for response in responses.iter() {
        let mut urls = url::UrlList::from_html(response.get_body());
        println!("Found {} links: adding new ones to queue", urls.len());
        url_q.add_url_list(&mut urls);
    }
}
