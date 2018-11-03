mod link;
mod report;
mod request;

/// Starts the crawling
pub fn run(start_urls: &mut Vec<String>) {
    let mut url_q = link::UrlQueue::new();
    let mut visited_q = link::UrlQueue::new();
    let mut responses: Vec<request::ResponseData> = vec![];

    url_q.add_vec(start_urls);

    loop {
        // make requests to all urls in queue
        println!("{} urls in queue: starting requests...", url_q.len());
        make_requests(&mut url_q, &mut visited_q, &mut responses);

        // extracting links from response data
        println!("Got {} responses: starting link search...", responses.len());
        extract_links(&mut responses, &mut url_q);

        // removing urls from queue which have recently been requested
        println!("{} urls in queue: removing recently used...", url_q.len());
        url_q.remove_from(&visited_q);
    }
}

fn make_requests(
    url_q: &mut link::UrlQueue,
    visited_q: &mut link::UrlQueue,
    responses: &mut Vec<request::ResponseData>,
) {
    for url in url_q.get_urls().iter() {
        println!("Sending request to {}", url);
        let response = request::get(url);
        if response.is_some() {
            responses.push(response.unwrap());
            println!("Request successful");
        }
        visited_q.add(url.to_string());
    }
}

fn extract_links(responses: &mut Vec<request::ResponseData>, url_q: &mut link::UrlQueue) {
    for response in responses.iter() {
        let mut links = link::process(&response.get_body());
        println!("Found {} links: adding new ones to queue...", links.len());
        url_q.add_vec(&mut links);
    }
}
