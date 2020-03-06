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

#[cfg(test)]
mod tests {
    use super::marshal_subreddit::*;
    use super::*;

    #[test]
    fn test_comment_data() {
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

        let comments = Comments {
            link: listing,
            comments: Listing {
                data: ListingData {
                    children: vec![Children {
                        kind: String::from("Comment-kind"),
                        data: ChildrenData {
                            id: String::from("Comment_id"),
                        },
                    }],
                },
            },
        };

        assert_eq!(comments.comments.data.children[0].data.id, "Comment_id")
    }
}
