mod lib;

use lib::run;

fn main() {
    let start_url = "http://www.berlin.de/polizei/polizeimeldungen/archiv";
    run(start_url);
}
