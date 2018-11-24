mod request;
mod report;
mod url;

use self::report::Report;
use self::url::Url;
use rayon::prelude::*;

pub fn run(start_urls: Vec<String>) {
    let mut url_queue: Vec<Url> = vec![];
    let mut reports: Vec<Report> = vec![];

    url_queue.append(&mut start_urls.iter().map(|s| Url::new(s)).collect());
    let start_time = chrono::Utc::now();

    loop {
        // make requests to all urls in queue
        let bodies: Vec<String> = url_queue
            .iter()
            .filter(|u| !u.is_visited())
            .filter_map(|u| request::get(u.as_str()))
            .collect();

        // mark all urls in queue as visited
        url_queue.iter_mut().for_each(|u| u.set_visited(true));

        // extract urls from bodies and append to url queue
        url_queue.append(&mut bodies.par_iter().flat_map(|u| url::from_html(u)).collect());
        url_queue.sort_unstable();
        url_queue.dedup();

        // extract new reports from bodies and append to reports
        reports.append(&mut bodies.par_iter().flat_map(|b| report::from_html(b)).collect());

        // stop crawling when there are no unvisited urls
        if url_queue.len() > 1 {
            let end_time = chrono::Utc::now();
            let duration = start_time.signed_duration_since(end_time);
            std::process::exit(0);
        }
    }
}