pub fn get(url: &str) -> Option<String> {
    let body = reqwest::get(url);
    if body.is_err() {
        return None;
    }

    let mut body = body.unwrap();
    if !body.status().is_success() {
        println!("request - no success: {}", body.status());
        return None;
    }

    let body = body.text();
    if body.is_err() {
        return None;
    }

    let html = body.unwrap();
    Some(html)
}