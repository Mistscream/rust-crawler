pub fn get(url: &str) -> Option<String> {
    println!("Sending get request to {}", &url);

    let body = reqwest::get(url);
    if body.is_err() {
        return None;
    }

    let mut body = body.unwrap();
    if !body.status().is_success() {
        println!("request - no success: {}", body.status());
        return None;
    }
    println!("Getting successful response from {}", &url);

    let body = body.text();
    if body.is_err() {
        return None;
    }

    let html = body.unwrap();
    Some(html)
}