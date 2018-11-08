mod http;
mod report;
mod url;

use self::http::Response;
use self::report::Report;
use self::url::{Url, UrlList};

pub fn run(start_url: &str) {
    let mut url_queue = UrlList::new();
    let mut responses: Vec<Response> = vec![];
    let mut reports: Vec<Report> = vec![];

    url_queue.add_url(Url::new(start_url));

    loop {
        // make requests to all urls in queue and store responses
        println!("{} urls in queue, sending requests", url_queue.len());
        let mut new_responses = send_request(&mut url_queue);
        println!("{} new responses", new_responses.len());
        responses.append(&mut new_responses);
        println!("{} responses all together", responses.len());

        // find urls from responses and append to url queue
        let mut new_urls = find_urls(&mut responses);
        println!("{} urls found", new_urls.len());
        url_queue.add_url_list(&mut new_urls);

        // parse relevant data from reports
        reports.append(&mut create_reports(&responses));

        // stop crawling when there are no new unvisited urls
        if url_queue.len() == 0 {
            println!("Responses: {}", responses.len());
            std::process::exit(0);
        }
    }
}

fn send_request(url_q: &mut UrlList) -> Vec<Response> {
    let mut responses: Vec<Response> = vec![];
    for url in url_q.get_urls_mut().iter_mut() {
        if !url.is_visited() {
            let response = http::get(url.get_string());
            if response.is_some() {
                responses.push(response.unwrap());
                url.set_visited(true);
            }
        }
    }

    responses
}

fn find_urls(responses: &mut Vec<Response>) -> UrlList {
    let mut urls = UrlList::new();
    for response in responses.iter() {
        let mut new_urls = UrlList::from_html(response.get_body());
        urls.add_url_list(&mut new_urls);
    }

    urls
}

fn create_reports(responses: &Vec<Response>) -> Vec<Report> {
    let reports: Vec<Report> = vec![];
    for response in responses {
        Report::new(response);
    }

    reports
}
