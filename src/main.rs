extern crate reqwest;
extern crate select;
extern crate regex;

use select::document::Document;
use select::predicate::{Attr, Name, And, Class};
use regex::{RegexBuilder, Regex, Captures};


fn get_current_dax() -> Vec<String> {
    let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
        .map_err(|_| ())
        .and_then(|r| Document::from_read(r)
            .map_err(|_| ()));

    if let Ok(d) = res {
        let pat = Regex::new(r"/aktie/(.*)-Aktie").unwrap();
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

fn get_name_ind() -> Vec<(String, String)> {
    let res = reqwest::get("http://www.boerse-online.de/index/liste/DAX")
        .map_err(|_| ())
        .and_then(|r| Document::from_read(r)
            .map_err(|_| ()));

    if res.is_err() {
        return Vec::new();
    }
    let d = res.unwrap();

    let pat = RegexBuilder::new(r"/aktie/(.*)-Aktie.*\n(.*\n)*?(\d+,\d+)")
        .multi_line(true)
        .build()
        .unwrap();

    let x: String = d.find(Class("table-hover"))
        .next()
        .map(|x| x.html())
        .unwrap_or(String::new());

    pat.captures_iter(&x)
        .map(|v| (String::from(&v[1]), String::from(&v[3])))
        .collect::<Vec<(String, String)>>()
}


fn main() {

    for (n, v) in get_name_ind() {
        println!("{} - {}", n, v);
    }

    /*
    for c in get_current_dax() {
        println!("{}", c);
    }
    */
}
