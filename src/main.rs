mod lib;

use lib::*;

fn main() {
    // let mut url_queue = vec![];
    // let mut responses = vec![];

    let url = "http://www.berlin.de/polizei/polizeimeldungen/archiv/";
    // url_queue.push(url);

    let response = request(&url);
    // responses.push(response);
    match response {
        Some(r) => {
            let links = find_links(&r.body);
            for link in links.into_iter() {
                println!("{}", link);
            }
        }
        None => ()
    }
}
