mod data;
mod webclient;

extern crate log;
//extern crate simple_logger;

use data::{string_manipulation, Comments, RedditSite, SerdeDeserializeObject};
use log::{error, info, LevelFilter};

fn main() {
    // sync post request of some json.

    simple_logger::init().unwrap();
    log::set_max_level(LevelFilter::Info);

    let url = webclient::create_url("/r/all/hot", Some("?&limit=5"));
    let body = webclient::gather_site(url);
    let all = RedditSite::new(&body);

    for children in &all.data.children {
        let permalink = &children.data.permalink;
        error!(
            "{}",
            string_manipulation::truncate(permalink.to_string(), 15)
        );
        let url = webclient::create_url(permalink, None);
        let body = webclient::gather_site(url);
        let comments = Comments::new(&body);
        info!(
            "{} has {} comments",
            comments.link.data.children[0].data.permalink,
            comments.comments.data.children.len()
        );
    }
}
