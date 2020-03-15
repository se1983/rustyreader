use log::error;

use super::data::{Comments, RedditSite, Unmarshal};


pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: Vec<(String, String)>,

    listings: Option<Comments>,
    request: Option<ureq::Request>,
    response: Option<ureq::Response>,
}

fn send_request(method: &str, url: &str, query: &str) -> ureq::Response {
    match method {
        "GET" => ureq::get(&url).query_str(&query).call(),
        _ => panic!("methode {} not defined!", method),
    }
}

impl RedditClient {
    pub fn new(subreddit: String) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit,
            queries: vec![],
            listings: None,
            request: None,
            response: None,
        }
    }

    fn add_query<'a>(&'a mut self, name: String, value: String) -> &'a mut Self {
        self.queries.push((name, value));
        self
    }

    pub fn limit<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.add_query(String::from("limit"), format!("{}", value))
    }

    fn download_reddit_site(&self) -> RedditSite {
        let url = format!("{}{}.json", self.base_url, self.subreddit);
        let query = match self.queries.len() {
            0 => String::from(""),
            _ => {
                let mut query = String::from("?");
                for qry in &self.queries {
                    query.push_str(&format!("{}={}&", qry.0, qry.1));
                }
                query
            }
        };
        let response = send_request("GET", &url, &query)
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
