use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub name: String,
    pub author: String,
    pub subreddit: String,
    pub ups: i32,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct Post {
    pub data: Data
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct PaginatedSite {
    pub children: Vec<Post>,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct RedditSite {
    pub data: PaginatedSite
}

