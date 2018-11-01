mod lib;

use lib::{run, ResponseData};

fn main() {
    let start_url = String::from("http://www.berlin.de/polizei/polizeimeldungen/archiv");
    let mut url_queue: Vec<String> = vec![start_url];
    let mut responses: Vec<ResponseData> = vec![];

    run(&mut url_queue, &mut responses);
}
