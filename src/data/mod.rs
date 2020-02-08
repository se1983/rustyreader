

pub mod marshal_json {
    use serde::{Deserialize, Serialize};
    use std::error::Error;

    #[derive(Serialize, Deserialize)]
    pub struct Data {
        pub title: String,
        pub name: String,
        pub author: String,
        pub subreddit: String,
        pub ups: i32,
    }


    #[derive(Serialize, Deserialize)]
    pub struct Post {
        pub data: Data
    }

    #[derive(Serialize, Deserialize)]
    pub struct PaginatedSite {
        pub children: Vec<Post>,
        pub after: String,
        pub before: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct RedditSite {
        pub data: PaginatedSite
    }

    pub fn serialize_redditpage(data: &String) -> Result<RedditSite, Box<dyn Error>> {
        let serialized_data: RedditSite = serde_json::from_str(data)?;
        Ok(serialized_data)
    }
}