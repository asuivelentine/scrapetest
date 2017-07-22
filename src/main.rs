extern crate reqwest;
extern crate select;
extern crate regex;

use select::document::Document;
use select::predicate::{Attr, Name, And};
use regex::RegexBuilder;
use regex::Regex;


fn get_current_dax() -> Vec<String> {
    let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
        .map_err(|_| ())
        .and_then(|r| Document::from_read(r)
            .map_err(|_| ()));

    if let Ok(d) = res {
        let pat  = Regex::new(r"/aktie/(.*)-Aktie").unwrap();
        let pat = RegexBuilder::new(r"/aktie/(.*)-Aktie")
            .multi_line(true)
            .build()
            .unwrap();

        let predicate = And(Name("a"), And(Attr("href", ()), Attr("title", ()))); 
        let x = d.find(predicate)
            .map(|d| d.html())
            .map(|h| pat.captures(&h)
                 .map(|c| c[1].to_string()))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect::<Vec<String>>();
        
        return x;
    }
    Vec::new()
}


fn main() {
    for c in get_current_dax() {
        println!("{}", c);
    }
}
