pub mod string_manipulation;
pub mod marshal_types;

use serde_json::error::Error;
use marshal_types::RedditSite;

// ToDo Check if impl RedditSite.serialize(data) is possible
pub fn serialize_redditpage(data: &String) -> Result<RedditSite, Error> {

    let serialized_data: RedditSite = serde_json::from_str(data)?;
    Ok(serialized_data)
}
