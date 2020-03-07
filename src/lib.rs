mod data;
use data::RedditSite;

#[allow(dead_code)]
pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: Vec<(String, String)>,

    listings: Option<RedditSite>,
}

#[allow(dead_code)]
impl RedditClient {
    pub fn new(subreddit: String) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit,
            queries: vec![],
            listings: None,
        }
    }

    fn add_query<'a>(&'a mut self, name: String, value: String) -> &'a mut Self {
        self.queries.push((name, value));
        self
    }

    pub fn limit<'a>(&'a mut self, value: i32) -> &'a mut Self {
        self.add_query(String::from("limit"), format!("{}", value))
    }
}
