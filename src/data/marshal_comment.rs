use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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
    pub link: Listing,
    pub comments: Listing,
}

pub fn serialize_comment(data: &String) -> Comments {
    let comments: Comments =
        serde_json::from_str(data).unwrap_or_else(|error| panic!("{:?}", error));

    comments
}
