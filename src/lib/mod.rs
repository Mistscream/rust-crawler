mod database;
mod report;
mod request;
mod url;

use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn run(urls: HashMap<String, bool>) {
    let mut url_queue = urls;
    let mut reports = Vec::new();

    let start = Utc::now();
    loop {
        // make requests to all urls in queue
        let mut bodies: Vec<String> = Vec::new();
        for (url, state) in url_queue.iter_mut() {
            if *state == false {
                match request::get(&url) {
                    Some(res) => {
                        bodies.push(res);
                        *state = true;
                    }
                    None => (),
                }
            }
        }

        // extract urls from bodies and append to url queue
        bodies
            .par_iter()
            .flat_map(|b| url::from_html(&b))
            .filter(|u| !url_queue.contains_key(u))
            .collect::<Vec<String>>()
            .iter()
            .for_each(|u| {
                url_queue.insert(String::from(u.as_str()), false);
            });

        // extract new reports from bodies and append to reports
        reports.append(
            &mut bodies
                .par_iter()
                .flat_map(|b| report::Report::from_html(b))
                .collect(),
        );

        // stop crawling when there are no unvisited urls
        if url_queue.iter().filter(|(_, v)| *v == &false).count() == 0 {
            break;
        }
    }

    let end = Utc::now();
    let time = end.signed_duration_since(start);
    let minutes = chrono::Duration::num_minutes(&time);
    println!("execution time: {}:{}", minutes / 60, minutes % 60);
    println!("reports found: {}", reports.len());
    println!("urls crawled (excluding report urls): {}", url_queue.len());
}
