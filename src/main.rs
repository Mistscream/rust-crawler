mod lib;

use lib::run;

fn main() {
    let start_url = String::from("http://www.berlin.de/polizei/polizeimeldungen/archiv");
    run(vec![start_url]);
}
