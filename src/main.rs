extern crate log;
extern crate rustyreader_lib as lib;

use lib::RedditClient;

fn main() {
    let subreddit = String::from("/r/programming/new");
    let comments = RedditClient::new(subreddit).limit(3).get();
    for comment in comments {
        println!("{}", comment.link.data.children[0].data.title);
    }
}
