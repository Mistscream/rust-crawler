extern crate chrono;
extern crate reqwest;
extern crate rayon;
extern crate select;
extern crate mongodb;

mod lib;

fn main() {
    let u1 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2014/?page_at_1_0=1");
    let u2 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2015/?page_at_1_0=1");
    let u3 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2016/?page_at_1_0=1");
    let u4 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2017/?page_at_1_0=1");
    let u5 = String::from("https://www.berlin.de/polizei/polizeimeldungen/archiv/2018/?page_at_1_0=1");
    let urls = vec![u1, u2, u3, u4, u5];
    lib::run(urls);
}
