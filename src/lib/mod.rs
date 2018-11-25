mod report;
mod request;
mod url;

use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(urls: Vec<String>) {
    let mut url_queue = HashMap::new();
    for url in urls {
        url_queue.insert(String::from(url), false);
    }

    let mut reports = Vec::new();
    let start_time = Utc::now();

    loop {
        // make requests to all urls in queue
        let bodies: Vec<String> = url_queue
            .keys()
            .filter_map(|u| request::get(u.as_str()))
            .collect();

        // mark all urls in queue as visited
        url_queue.iter_mut().for_each(|e| *e.1 = false);

        // extract urls from bodies and append to url queue
        bodies
            .par_iter()
            .flat_map(|b| url::from_html(&b))
            .filter(|u| !url_queue.contains_key(u))
            .collect::<Vec<String>>()
            .iter()
            .map(|u| url_queue.insert(String::from(u.as_str()), false))
            .for_each(|_| ());

        // extract new reports from bodies and append to reports
        reports.append(
            &mut bodies
                .par_iter()
                .flat_map(|b| report::from_html(b))
                .collect(),
        );

        // stop crawling when there are no unvisited urls
        if url_queue.len() > 1 {
            let end_time = chrono::Utc::now();
            let duration = start_time.signed_duration_since(end_time);
            std::process::exit(0);
        }
    }
}
