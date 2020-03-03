use serde::Deserialize;

pub trait SerdeDeserializeObject {
    fn new<'de>(data: &'de str) -> Self
    where
        Self: Deserialize<'de>,
    {
        let serialized_data: Self = serde_json::from_str(&data).unwrap();
        serialized_data
    }
}

pub mod marshal_comment;
pub mod marshal_subreddit;
pub mod string_manipulation;

pub use marshal_comment::serialize_comment;
pub use marshal_subreddit::RedditSite;
