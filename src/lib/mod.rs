mod database;
mod report;
mod request;
mod url;

use chrono::Utc;
use rayon::prelude::*;
use std::collections::HashMap;
use lib::report::Report;

pub fn run(urls: HashMap<String, bool>) {
    let mut url_queue = urls;
    let mut mongo = database::MongoDB::new("localhost", 27017).unwrap();
    let mut rep_count = 0;

    let start = Utc::now();
    loop {
        println!(
            "unvisited urls in queue: {}",
            url_queue.iter().filter(|(_, v)| *v == &false).count()
        );
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
        for body in bodies.iter() {
            let urls = url::from_html(&body);
            for url in urls {
                if !url_queue.contains_key(&url) {
                    url_queue.insert(String::from(url), false);
                }
            }
        }

        // extract new reports from bodies and append to reports
        let reports: Vec<Report> = bodies
            .iter()
            .flat_map(|b| Report::from_html(b))
            .collect();

        // store reports in database
        for report in reports {
            mongo.add(report);
            rep_count += 1;
        }

        // stop crawling when there are no unvisited urls
        if url_queue.iter().filter(|(_, v)| *v == &false).count() == 0 {
            break;
        }
    }

    let end = Utc::now();
    let time = end.signed_duration_since(start);
    let minutes = chrono::Duration::num_minutes(&time);
    println!("execution time: {}:{}", minutes / 60, minutes % 60);
    println!("reports found: {}", rep_count);
    println!("urls crawled (excluding report urls): {}", url_queue.len());
}
