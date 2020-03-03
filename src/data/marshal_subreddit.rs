use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub trait SerdeDeserializeObject {
    fn new<'de>(data: &'de str) -> Self
    where
        Self: Deserialize<'de>,
    {
        let serialized_data: Self = serde_json::from_str(&data).unwrap();
        serialized_data
    }
}

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

impl SerdeDeserializeObject for RedditSite {}
