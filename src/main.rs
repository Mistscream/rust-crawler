extern crate chrono;
extern crate reqwest;
extern crate rayon;
extern crate select;
extern crate mongodb;

mod lib;

use std::collections::HashMap;

fn main() {
    let u1 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2014/?page_at_1_0=1");
    let u2 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2015/?page_at_1_0=1");
    let u3 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2016/?page_at_1_0=1");
    let u4 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2017/?page_at_1_0=1");
    let u5 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2018/?page_at_1_0=1");

    let mut urls: HashMap<String, bool> = HashMap::new();
    urls.insert(u1, false);
    urls.insert(u2, false);
    urls.insert(u3, false);
    urls.insert(u4, false);
    urls.insert(u5, false);

    lib::run(urls);
}
