use log::error;

use super::data::{Comments, RedditSite, Unmarshal};


pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: String,
}

fn send_request(method: &str, url: &str, query: &str) -> ureq::Response {
    let response = match method {
        "GET" => {
            ureq::get(&url)
                .query_str(&query)
                .timeout_read(500)
                .call()
        },
        _ => panic!("methode {} not defined!", method),
    };
    assert!(response.ok());

    response
}

impl RedditClient {
    pub fn new(subreddit: String) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit,
            queries: String::from(""),
        }
    }

    fn add_query<'a>(&'a mut self, name: String, value: String) -> &'a mut Self {
        self.queries = match self.queries.as_str() {
            "" => format!("?{}={}", name, value),
            _ => format!("{}&{}={}", &self.queries, name, value)
        };
        self
    }

    pub fn limit<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.add_query(String::from("limit"), format!("{}", value))
    }

    fn download_reddit_site(&self) -> RedditSite {
        let url = format!("{}{}.json", self.base_url, self.subreddit);
        let response = send_request("GET", &url, &self.queries)
            .into_string()
            .unwrap_or_else(|error| {
                // ToDo Handle error better -- don't panic!
                error!("{}", error);
                panic!("{:?}", error)
            });
        RedditSite::new(&response)
    }

    fn download_listings(&self, site: RedditSite) -> Vec<Comments> {
        let mut listings = vec![];
        for post in site.data.children {
            listings.push(Comments::new(
                &send_request(
                    "GET",
                    &format!("{}{}.json", self.base_url, post.data.permalink),
                    "",
                )
                    .into_string()
                    .unwrap(),
            ));
        }

        listings
    }

    pub fn get<'a>(&'a mut self) -> Vec<Comments> {
        let reddit_site = self.download_reddit_site();
        self.download_listings(reddit_site)
    }
}
