extern crate reqwest;
extern crate select;
extern crate regex;

use select::document::Document;
use select::predicate::{Attr, Name, And};
use regex::Regex;


fn run() -> Result<(), ()> {
    let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
        .map_err(|e| ())
        .and_then(|r| Document::from_read(r)
            .map_err(|e| ()));

    if let Ok(d) = res {
        let pat  = Regex::new(r"/aktie/(.*)-Aktie").unwrap();
        let predicate = And(Name("a"), And(Attr("href", ()), Attr("title", ()))); 
        let x = d.find(predicate)
            .map(|d| d.html())
            .map(|h| pat.captures(&h)
                 .map(|c| c[0].to_string()))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect::<Vec<String>>();

        
        for w in x {
            println!("{:?}", w);
        }
    }
    Ok(())
}


fn main() {
    run();
}
