extern crate reqwest;

use self::reqwest::*;

pub fn get_request(url: &str) {
    let response = get(url).expect("Could not send request");
    println!("status: {}", response.status());
}

