mod lib;

use lib::*;

fn main() {
    // let mut url_queue = vec![];
    // let mut responses = vec![];

    let url = "http://www.berlin.de/polizei/polizeimeldungen/archiv/";
    // url_queue.push(url);

    let response = request(&url);
    // responses.push(response);
    println!("{:?}", response);
}
