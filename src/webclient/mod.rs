use std::option::Option;
use ureq;

pub fn create_url(subreddit: &str, optional_args: Option<String>) -> String {
    let args = optional_args.unwrap_or(String::from(""));
    format!("https://www.reddit.com/r/{}.json?{}", subreddit, args)
}

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
    let resp = req.send();
    // ToDo logging
    println!("HTTP Status: {}", resp.status());

    let text = resp.into_string().unwrap_or_else(|error| {
        // ToDo Handle error better -- don't panic!
        panic!("{:?}", error)
    });

    // ToDo logging
    // println!("{}", text);
    text
}
