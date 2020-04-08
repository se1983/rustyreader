use super::data::{Comments, RedditSite, Unmarshal};
use super::requests;


pub struct RedditClient {
    base_url: String,
    subreddit: String,
    queries: String,
    webclient: requests::Cacher,

}


impl RedditClient {
    pub fn new(subreddit: &str) -> Self {
        Self {
            base_url: String::from("https://reddit.com"),
            subreddit: String::from(subreddit),
            queries: String::from(""),
            webclient: requests::Cacher::new(|url| {
                ureq::get(url).call().into_string().unwrap()
            }),
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

    fn download_reddit_site(&mut self) -> RedditSite {
        let url = format!("{}{}.json{}", self.base_url, self.subreddit, self.queries);
        let data = self.webclient.data(&url);
        RedditSite::new(&data)
    }

    fn download_listings(&mut self, site: RedditSite) -> Vec<Comments> {
        let mut listings = vec![];
        for post in site.data.children {
            let url = format!("{}{}.json", self.base_url, post.data.permalink);
            listings.push(Comments::new(
                &self.webclient.data(&url)
            ));
        }
        listings
    }

    pub fn get<'a>(&'a mut self) -> Vec<Comments> {
        let reddit_site = self.download_reddit_site();
        self.download_listings(reddit_site)
    }
}
