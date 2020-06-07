use reddit::RedditClient;

mod reddit;

fn do_print(post: &str){
   println!("------------------------------------------------------------------\n------------------------------------------------------------------\n{}\n\n", post)
}

pub fn run() {

    let searchwords = vec!("Rust", "skajsdkahk");
    let subreddit = "/r/rust";
    let do_sth = do_print;

    for (_, listing) in RedditClient::new(subreddit).limit(100).enumerate() {
        for l in listing.comments.data.children.iter()
            .filter(|x| x.data.body.is_some())
            .map(|x| x.data.body.as_ref().unwrap())
            .filter(|x| searchwords.iter().any(|y| x.contains(y))) {
           do_sth(l);
        }
    };
}
