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

    println!("Starting...");
    let mut i = 0;
    loop {
        // make requests to all urls in queue
        // save response bodies in Vec
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

        // lets print some useful info
        println!(
            "visited urls in queue:\t{}",
            url_queue.iter().filter(|(_, v)| *v == &true).count()
        );
        println!(
            "unvisited urls in queue:\t{}",
            url_queue.iter().filter(|(_, v)| *v == &false).count()
        );
        println!("found reports:\t\t{}", reports.len());

        // stop crawling when there are no unvisited urls
        if url_queue.iter().filter(|(_, v)| !*v).count() > 0 {
            break;
        }
        i += 1;
    }

    let end_time = Utc::now();
    let duration = end_time.signed_duration_since(start_time);
    println!("execution time: {}", duration);
    println!("reports found: {}", reports.len());
    println!("urls crawled (excluding report urls): {}", url_queue.len());
    println!("execution loops: {}", i);

    std::process::exit(0);
}
