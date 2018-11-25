mod report;
mod request;
mod url;

use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(urls: Vec<String>) {
    let start_time = Utc::now();
    let mut url_queue = HashMap::new();
    let mut reports = Vec::new();

    for url in urls {
        url_queue.insert(String::from(url), false);
    }

    loop {
        // make requests to all urls in queue
        // save response bodies in Vec
        let bodies = url_queue
            .par_iter()
            .map(|(k, _)| k)
            .filter_map(|u| request::get(u.as_str()))
            .collect::<Vec<String>>();

        // mark all urls in queue as visited
        url_queue.iter_mut().for_each(|(_, v)| *v = false);

        // extract urls from bodies and append to url queue
        bodies
            .par_iter()
            .flat_map(|b| url::from_html(&b))
            .filter(|u| !url_queue.contains_key(u))
            .collect::<Vec<String>>()
            .iter()
            .for_each(|u| {
                url_queue.insert(String::from(u.as_str()), false);
                ()
            });

        // extract new reports from bodies and append to reports
        reports.append(
            &mut bodies
                .par_iter()
                .flat_map(|b| report::from_html(b))
                .collect(),
        );

        // stop crawling when there are no unvisited urls
        if url_queue.iter().filter(|(_, v)| !*v).count() > 1 {
            break;
        }
    }

    let end_time = Utc::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("execution time: {}", duration);
    println!("reports found: {}", reports.len());
    println!("urls crawled (excluding report urls): {}", url_queue.len());

    std::process::exit(0);
}
