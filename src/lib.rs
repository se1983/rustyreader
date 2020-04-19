use std::sync::{Arc, Mutex};
use std::thread;

use reddit::RedditClient;
use reddit::requests;

mod reddit;


fn find_word(listings: &mut RedditClient, word: &str) {
    for (_, listing) in listings.enumerate() {
        for comment in listing.comments.data.children.iter()
            .into_iter()
            .filter(|x| x.data.body.is_some())
            .map(|x| x.data.body.as_ref().unwrap())
            .filter(|x| x.contains(&word)) {
            println!("\n\n\n{comment:.max_width$}\nhttps://reddit.com{link}\n\n",
                     comment = comment,
                     link = listing.link.data.children[0].data.permalink,
                     max_width = 60
            );
        }
    };
}


pub fn run() {
    let cacher = Arc::new(Mutex::new(
        requests::Cacher::new(requests::get_request())
    ));

    let mut children = vec![];

    for (subreddit, word) in vec![
        ("/r/all/new", "Trump"),
        ("/r/all/new", "Corona"),
        ("/r/programming", "Rust")] {
        let handle = thread::spawn(move || {
            find_word(RedditClient::new(subreddit, Arc::clone(&cacher)).limit(100), word);
        });

        children.push(handle);
    }
}
