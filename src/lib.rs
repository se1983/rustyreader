use reddit::RedditClient;

mod reddit;

pub fn run() {

    let searchwords = vec!("Rust", "skajsdkahk");
    let subreddit = "/r/rust";

    for (_, listing) in RedditClient::new(subreddit).limit(100).enumerate() {
        for l in listing.comments.data.children.iter()
            .filter(|x| x.data.body.is_some())
            .map(|x| x.data.body.as_ref().unwrap())
            .filter(|x| searchwords.iter().any(|y| x.contains(y))) {
            println!("{}", l);
        }
    };
}
