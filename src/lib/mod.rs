mod link;
mod report;
mod request;

/// Starts the crawling
pub fn run(start_urls: &mut Vec<String>) {
    let mut url_queue = link::UrlQueue::new();
    let mut responses: Vec<request::ResponseData> = vec![];

    url_queue.add(start_urls);

    loop {
        println!("Got {} urls in queue: starting requests...", url_queue.len());
        for url in url_queue.get().iter() {
            println!("Sending request to {}", url);
            let response = request::get(url);
            if response.is_some() {
                responses.push(response.unwrap());
                println!("Request successful");
            }
        }

        println!("Got {} responses: starting link search...", responses.len());
        for response in responses.iter() {
            let mut links = link::process(response.get_body());
            println!("Found {} new links", links.len());
            url_queue.add(&mut links);
        }
    }
}
