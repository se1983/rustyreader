use std::option::Option;
use ureq;

use log::{info, trace};

pub fn create_url(permalink: &str, optional_args: Option<&str>) -> String {
    // TODO URL builder  https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
    // - optional args
    // - optional json
    let args = optional_args.unwrap_or("");
    format!("https://www.reddit.com{}.json{}", permalink, args)
}

#[allow(dead_code)]
struct Request {
    methode: String,
    url: String,
    body: Option<String>,
    header: Option<String>,
}

impl Request {
    pub fn send(&self) -> ureq::Response {
        match self.methode.as_str() {
            "GET" => ureq::get(&self.url).call(),
            "POST" => ureq::post(&self.url).call(),
            _ => panic!("methode {} not defined!", self.methode),
        }
    }
}

pub fn gather_site(url: String) -> String {
    let req = Request {
        methode: String::from("GET"),
        url,
        body: None,
        header: None,
    };
    info!("({})", req.methode);
    info!("({})", req.url);
    let resp = req.send();
    info!("[{}] ", resp.status());
    assert!(resp.ok());

    let text = resp.into_string().unwrap_or_else(|error| {
        // ToDo Handle error better -- don't panic!
        panic!("{:?}", error)
    });

    trace!("{}", text);
    text
}
