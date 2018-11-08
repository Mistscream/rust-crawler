extern crate chrono;
extern crate regex;
extern crate reqwest;
extern crate select;

mod lib;

fn main() {
    let start_url = "https://www.berlin.de/polizei/polizeimeldungen/archiv/";
    lib::run(start_url);
}
