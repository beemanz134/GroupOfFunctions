use std::io;
use reqwest::blocking::get;
use select::document::Document;
use select::predicate::Name;
use regex::Regex;

pub fn start() {
    let url_regex = Regex::new(r"^(http|https)://[^\s/$.?#].[^\s]*$").unwrap(); // Simple URL regex

    loop {
        let mut input = String::new();
        println!("=======Please enter url======");
        println!("enter 0 to exit");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim() == "0" {
            break;
        }

        let url = input.trim().to_string();
        if !url_regex.is_match(&url) {
            println!("Invalid URL: {}", url);
            continue;
        }

        println!("Fetch url: {}", url);
        match get(&url) {
            Ok(response) => {
                let body = response.text().expect("Failed to read response body");

                let document = Document::from(body.as_str());
                for node in document.find(Name("a")) {
                    if let Some(href) = node.attr("href") {
                        println!("Found link: {}", href);
                    }
                }
            }
            Err(e) => {
                println!("Error fetching URL: {}", e);
            }
        }
    }
}