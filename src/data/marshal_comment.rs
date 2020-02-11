use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use log;

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

pub fn serialize_comment(data: &String) -> Comments {
    let comment_page: Comments =
        serde_json::from_str(data).unwrap_or_else(|error| panic!("{:?}", error));
    log::info!(
        "Found {} comments on {}",
        comment_page.comments.data.children.len(),
        comment_page.link.data.children[0].data.permalink,
    );
    comment_page
}
