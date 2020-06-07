use reddit::RedditClient;

mod reddit;

pub fn run() {

    let searchwords = vec!("Rust", "skajsdkahk");

    for (_, listing) in RedditClient::new("/r/rust").limit(100).enumerate() {
        for l in listing.comments.data.children.iter()
            .into_iter()
            .filter(|x| x.data.body.is_some())
            .map(|x| x.data.body.as_ref().unwrap())
            .filter(|x| searchwords.iter().any(|y| x.contains(y))) {
            println!("{}", l);
        }
    };
}
