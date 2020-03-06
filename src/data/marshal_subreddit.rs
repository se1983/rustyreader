use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub name: String,
    pub author: String,
    pub subreddit: String,
    pub ups: i32,
    pub permalink: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub children: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditSite {
    pub data: Links,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let listing = RedditSite {
            data: Links {
                children: vec![Post {
                    data: Data {
                        title: String::from("Title"),
                        name: String::from("Name"),
                        author: String::from("Author"),
                        subreddit: String::from("Subreddit"),
                        ups: 2,
                        permalink: String::from("Permalink"),
                    },
                }],
            },
        };

        assert_eq!(listing.data.children[0].data.title, "Title");
    }
}
