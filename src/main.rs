mod data;
mod webclient;

use data::string_manipulation;

fn main() {
    // sync post request of some json.

        let url = webclient::create_url("rust", None);
        let body = webclient::gather_site(url);
        let all = data::serialize_redditpage(
            &body).unwrap_or_else(|error|{
            panic!("{:?}", error)
        });

        for post in all.data.children {
            println!("- \t{who:<20} wrote {what:<50} in {place} and got {ups:^5} upvotes. [{name}]",
                     who = string_manipulation::truncate(post.data.author, 20),
                     what = string_manipulation::truncate(post.data.title, 50),
                     place = post.data.subreddit,
                     ups = post.data.ups,
                     name = post.data.name,
            );
        }

}


