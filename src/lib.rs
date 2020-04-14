use reddit::RedditClient;

mod reddit;

pub fn run() {
    for (_, listing) in RedditClient::new("/r/rust").limit(100).enumerate() {
        for l in listing.comments.data.children.iter()
            .into_iter()
            .filter(|x| x.data.body.is_some())
            .map(|x| x.data.body.as_ref().unwrap())
            .filter(|x| x.contains("Rust")) {
            println!("{}", l);
        }
    };
}
