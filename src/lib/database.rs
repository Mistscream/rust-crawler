use lib::report::Report;
use mongodb::db::ThreadedDatabase;
use mongodb::{bson, doc, Bson};
use mongodb::{Client, ThreadedClient};

pub struct MongoDB {
    client: mongodb::Client,
    address: String,
    port: u16,
    collection: mongodb::coll::Collection,
}

impl MongoDB {
    pub fn new(addr: &str, port: u16) -> Option<MongoDB> {
        let client = Client::connect(addr, port);
        if client.is_err() {
            return None;
        }

        let client = client.unwrap();
        let coll = client.db("test").collection("movies");

        Some(MongoDB {
            client: client,
            address: String::from(addr),
            port: port,
            collection: coll,
        })
    }

    pub fn add(&mut self, report: Report) {
        let doc = doc!{
            "url": report.get_fields()[0],
            "title": report.get_fields()[1],
            "date": report.get_fields()[2],
            "location": report.get_fields()[3],
            "text": report.get_fields()[4]
        };
        self.collection.insert_one(doc.clone(), None);
    }
}
