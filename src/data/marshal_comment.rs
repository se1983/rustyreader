use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::marshal_subreddit;

#[derive(Debug, Serialize, Deserialize)]
pub struct ChildrenData {
    pub id: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: ChildrenData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingData {
    pub children: Vec<Children>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub data: ListingData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    pub link: marshal_subreddit::RedditSite,
    pub comments: Listing,
}
