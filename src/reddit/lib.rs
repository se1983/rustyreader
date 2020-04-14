use std::collections::HashMap;

use super::data::{Comments, RedditSite, Unmarshal};
use super::requests;

struct Queries {
    queries: HashMap<String, String>,
}

impl Queries {
    pub fn new() -> Self {
        Queries { queries: HashMap::new() }
    }

    pub fn as_string(&self) -> String {
        let mut query_string = String::new();
        for (k, v) in &self.queries {
            query_string = match query_string.as_str() {
                "" => format!("?{}={}", k, v),
                _ => format!("{}&{}={}", &query_string, k, v)
            };
        }
        query_string
    }

    pub fn add(&mut self, k: String, v: String) {
        self.queries.insert(k, v);
    }
}

pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: Queries,
    webclient: requests::Cacher,
    position: usize,
}


impl RedditClient {
    pub fn new(subreddit: &str) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit: String::from(subreddit),
            queries: Queries::new(),
            webclient: requests::Cacher::new(requests::get_request()),
            position: 0,
        }
    }


    pub fn limit<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.queries.add(String::from("limit"), format!("{}", value));
        self
    }
}


impl Iterator for RedditClient {
    type Item = Comments;
    fn next(&mut self) -> Option<Self::Item> {
        let url = format!("{}{}.json{}", self.base_url, self.subreddit, self.queries.as_string());
        let data = self.webclient.data(&url);
        let site = RedditSite::new(&data);
        if self.position >= site.data.children.len() {
            return None
        };

        let url = format!(
            "{}{}.json", self.base_url, site.data.children[self.position].data.permalink
        );
        let listing = Comments::new(
            &self.webclient.data(&url)
        );
        self.position += 1;
        Some(listing)
    }
}