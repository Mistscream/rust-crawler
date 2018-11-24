extern crate chrono;
extern crate regex;
extern crate reqwest;
extern crate rayon;
extern crate scraper;

mod lib;

fn main() {
    let u1 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2014");
    let u2 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2015");
    let u3 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2016");
    let u4 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2017");
    let u5 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2018");
    let urls = vec![u1, u2, u3, u4, u5];
    // let urls = vec![u1];
    // nur archiv seiten durchsuchen!!!
    lib::run(urls);
}
