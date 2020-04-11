use super::data::{Comments, RedditSite, Unmarshal};
use super::requests;


pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: String,
    webclient: requests::Cacher,
    position: usize,
}

impl Iterator for RedditClient {
    type Item = Comments;
    fn next(&mut self) -> Option<Self::Item> {
        let url = format!("{}{}.json{}", self.base_url, self.subreddit, self.queries);
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


impl RedditClient {
    pub fn new(subreddit: &str) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit: String::from(subreddit),
            queries: String::from(""),
            webclient: requests::Cacher::new(requests::get_request()),
            position: 0,
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


}
