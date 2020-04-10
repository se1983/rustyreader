mod reddit;

use reddit::RedditClient;


pub fn run() {
    for (i, listing) in RedditClient::new("/r/programming").limit(300).enumerate() {
        println!("\n\n'[{}]\t{}' has {} comments.\n\t\thttps://reddit.com{}\n\t ",
                 i,
                 listing.link.data.children[0].data.title,
                 listing.comments.data.children.len(),
                 listing.link.data.children[0].data.permalink,
        );
    };
}
