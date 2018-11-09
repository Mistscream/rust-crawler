mod http;
mod report;
mod url;

use self::http::Response;
use self::report::Report;
use self::url::Url;

pub fn run(start_url: &str) {
    let mut url_queue: Vec<Url> = vec![];
    let mut responses: Vec<Response> = vec![];
    let mut reports: Vec<Report> = vec![];

    url_queue.push(Url::new(start_url));

    loop {
        // make requests to all urls in queue and store responses
        // let mut new_responses = send_request_conc(&mut url_queue);
        let mut new_responses = send_request(&mut url_queue);
        responses.append(&mut new_responses);

        // find urls from responses and append to url queue, remove dups
        let mut new_urls = find_urls(&mut responses);
        url_queue.append(&mut new_urls);
        url_queue.sort_unstable();
        url_queue.dedup();

        // parse relevant data from reports
        reports.append(&mut create_reports(&responses));

        // stop crawling when there are no new unvisited urls
        if url_queue.len() == 0 {
            println!("Responses: {}", responses.len());
            std::process::exit(0);
        }
    }
}

fn send_request(url_q: &mut Vec<Url>) -> Vec<Response> {
    println!("{} urls in queue, sending requests", url_q.len());

    let responses: Vec<Response> = url_q
        .iter()
        .filter(|u| !u.is_visited())
        .map(|u| http::get(u.get_string()))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();

    for url in url_q.iter_mut() {
        url.set_visited(true);
    }

    println!("{} new responses", responses.len());
    responses
}

fn find_urls(responses: &mut Vec<Response>) -> Vec<Url> {
    let mut urls: Vec<Url> = vec![];
    for response in responses.iter() {
        let mut new_urls = url::from_html(response.get_body());
        urls.append(&mut new_urls);
    }

    println!("{} urls found", urls.len());
    urls
}

fn create_reports(responses: &Vec<Response>) -> Vec<Report> {
    let reports: Vec<Report> = vec![];
    for response in responses {
        Report::new(response);
    }

    reports
}
