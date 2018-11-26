pub fn get(url: &str) -> Option<String> {
    println!("Sending get request to {}", url);
    let body = reqwest::get(url);
    if body.is_err() {
        return None;
    }

    let mut body = body.unwrap();
    if !body.status().is_success() {
        return None;
    }

    let body = body.text();
    if body.is_err() {
        return None;
    }

    let html = body.unwrap();
    println!("got successful response from {}", url);

    Some(html)
}