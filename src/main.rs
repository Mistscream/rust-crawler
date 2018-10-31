mod lib;

use lib::*;

fn main() {
    let url = "http://www.berlin.de/polizei/polizeimeldungen/archiv/";
    
    get_request(&url);
}
