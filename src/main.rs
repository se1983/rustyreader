mod data;
mod webclient;

extern crate log;
extern crate simple_logger;

use data::string_manipulation;
use data::{RedditSite, SerdeDeserializeObject};
use log::{info, LevelFilter};

#[allow(dead_code)]
fn print_page(posts: RedditSite) {
    for post in posts.data.children {
        println!(
            "- \t{who:<8} wrote {what:<70} in {place} and got {ups:^5} upvotes. [{name}]\n\t > https://reddit.com{permalink}",
            who = string_manipulation::truncate(post.data.author, 8),
            what = string_manipulation::truncate(post.data.title, 70),
            place = post.data.subreddit,
            ups = post.data.ups,
            name = post.data.name,
            permalink = post.data.permalink
        );
    }
}

fn main() {
    // sync post request of some json.

    simple_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);

    let url = webclient::create_url("/r/all/hot", Some("?&limit=3"));
    let body = webclient::gather_site(url);
    let all = data::RedditSite::new(&body);

    for children in &all.data.children {
        let permalink = &children.data.permalink;
        let url = webclient::create_url(permalink, None);
        let body = webclient::gather_site(url);
        let comments = data::serialize_comment(&body);
        info!(
            "comments of {}",
            comments.link.data.children[0].data.permalink
        );
        for comment in comments.comments.data.children {
            println!("{}", comment.data.id);
        }
    }
}
