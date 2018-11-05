mod link;
mod report;
mod request;

pub fn run(start_urls: &mut Vec<String>) {
    let mut url_q = link::UrlQueue::new();
    let mut visited_q = link::UrlQueue::new();
    let mut respones_list = request::ResponseList::new();

    url_q.add_vec(start_urls);

    loop {
        // make requests to all urls in queue
        println!("{} urls in queue: starting requests...", url_q.len());
        make_requests(&mut url_q, &mut visited_q, &mut respones_list);

        // extracting links from response data
        println!("Got {} responses: starting link search...", respones_list.len());
        extract_links(&mut url_q, &mut respones_list);

        // removing urls from queue which have recently been requested
        println!("{} urls in queue: removing recently used...", url_q.len());
        url_q.remove_from(&visited_q);
    }
}

fn make_requests(
    url_q: &mut link::UrlQueue,
    visited_q: &mut link::UrlQueue,
    responses: &mut request::ResponseList,
) {
    for url in url_q.get_urls().iter() {
        println!("Sending request to {}", url);
        let response = request::get(url);
        if response.is_some() {
            responses.add_response(response.unwrap());
            println!("Request successful");
        }
        visited_q.add(url.to_string());
    }
}

fn extract_links(url_q: &mut link::UrlQueue, responses: &mut request::ResponseList) {
    for response in responses.get_responses().iter() {
        let mut links = link::process(&response.get_body());
        println!("Found {} links: adding new ones to queue...", links.len());
        url_q.add_vec(&mut links);
    }
}
