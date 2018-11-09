mod http;
mod report;
mod url;

use self::http::Response;
use self::report::Report;
use self::url::Url;
use chrono::prelude::*;
use rayon::prelude::*;

pub fn run(start_url: &str) {
    let mut url_queue: Vec<Url> = vec![];
    let mut responses: Vec<Response> = vec![];
    let mut reports: Vec<Report> = vec![];

    url_queue.push(Url::new(start_url));
    let start_time = chrono::Utc::now();

    loop {
        // make requests to all urls in queue and store responses
        let mut new_responses = send_request(&mut url_queue);
        responses.append(&mut new_responses);

        // find urls from responses and append to url queue, remove dups
        let mut new_urls = find_urls(&mut responses);
        url_queue.append(&mut new_urls);
        url_queue.sort_unstable();
        url_queue.dedup();

        // parse relevant data from reports
        reports.append(&mut create_reports(&mut responses));

        // stop crawling when there are no new unvisited urls
        if url_queue.len() == 0 {
            let end_time = chrono::Utc::now();
            let duration = start_time.signed_duration_since(end_time);

            println!("Responses: {}", responses.len());
            println!("Time: {}min", duration.num_minutes());
            std::process::exit(0);
        }
    }
}

fn send_request(url_q: &mut Vec<Url>) -> Vec<Response> {
    println!("{} urls in queue, sending requests", url_q.len());

    let responses: Vec<Response> = url_q
        .par_iter()
        .filter(|u| !u.is_visited())
        .map(|u| http::get(u.get_string()))
        .filter(|r| r.is_some())
        .map(|r| r.unwrap())
        .collect();

    url_q.iter_mut().for_each(|u| u.set_visited(true));

    println!("{} new responses", responses.len());
    responses
}

fn find_urls(responses: &mut Vec<Response>) -> Vec<Url> {
    println!("{} responses in list, searching urls", responses.len());

    let urls: Vec<Url> = responses
        .par_iter()
        .flat_map(|u| url::from_html(u.get_body()))
        .collect();

    println!("{} urls found", urls.len());
    urls
}

fn create_reports(responses: &mut Vec<Response>) -> Vec<Report> {
    let reports: Vec<Report> = responses.par_iter().map(|r| Report::new(&r)).collect();

    reports
}
