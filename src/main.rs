extern crate reqwest;
extern crate select;
extern crate regex;

use select::document::Document;
use select::predicate::{Attr, Name, And};


fn run() -> Result<(), ()> {
    let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
        .map_err(|e| ())
        .and_then(|r| Document::from_read(r)
            .map_err(|e| ()));

    if let Ok(d) = res {
        let predicate = And(Name("a"), And(Attr("href", ()), Attr("title", ()))); 
        let x = d.find(predicate)
            .map(|d| d.html())
            .filter(|x| x.contains("/aktie/"))
            .collect::<Vec<String>>();

        
        for w in x {
            println!("{:?}", w);
        }

        for link in d.find(predicate) {
            println!("{}", link.html());
        }
    }
    Ok(())
}


fn main() {
    run();
}
