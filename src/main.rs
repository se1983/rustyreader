extern crate log;
extern crate rustyreader_lib as lib;

use lib::RedditClient;

fn main() {
    RedditClient::new(String::from("/r/all/new")).limit(5);
}
