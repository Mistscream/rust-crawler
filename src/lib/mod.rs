mod request;
mod link;
mod report;

use self::request::{ResponseData, request};
use self::link::process_links;

/// Starts the crawling
pub fn run(url: String) {
    let mut url_queue: Vec<String> = vec![url];
    let mut responses: Vec<ResponseData> = vec![];

    loop {
        println!("{} urls in queue: starting requests...", url_queue.len());
        for url in url_queue.iter() {
            println!("Sending request to {}", url);
            let response = request(url);
            match response {
                Some(r) => {
                    responses.push(r);
                    println!("Request successful");
                }
                None => println!("Request failed")
            }
        }

        println!(
            "{} responses from requests: starting link search...",
            responses.len()
        );
        for response in responses.iter() {
            let links = process_links(response.get_body());
            url_queue.extend(links);
            url_queue.sort_unstable();
            url_queue.dedup();
        }
    }
}
