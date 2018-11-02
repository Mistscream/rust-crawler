mod request;
mod link;
mod report;


/// Starts the crawling
pub fn run(start_urls: Vec<String>) {
    let mut url_queue = link::UrlQueue::new();
    let mut responses: Vec<request::ResponseData> = vec![];

    url_queue.add(start_urls);

    loop {
        println!("{} urls in queue: starting requests...", url_queue.len());
        for url in url_queue.get().iter() {
            println!("Sending request to {}", url);
            let response = request::get(url);
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
            let links = link::process(response.get_body());
            url_queue.add(links);
        }
    }
}
