use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::SerdeDeserializeObject;

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
