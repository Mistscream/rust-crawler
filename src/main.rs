mod lib;

use lib::*;

fn main() {
    let url = "http://www.berlin.de/polizei/polizeimeldungen/archiv/";

    let response = get_request(&url);
    match response {
        Some(r) => print_response(r),
        None => (),
    }
}
