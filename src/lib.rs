extern crate reqwest;

use self::reqwest::*;

pub struct HttpResponse {
    status: String,
    body: String,
}

pub fn get_request(url: &str) -> Option<HttpResponse> {
    let mut response = get(url);
    match response {
        Ok(mut res) => Some(HttpResponse {
            status: res.status().to_string(),
            body: res.text().unwrap(),
        }),
        Err(_) => {
            None
        }
    }
}

pub fn print_response(res: HttpResponse) {
    println!("Status: {}", res.status);
    println!("Body: {}", res.body);
}
