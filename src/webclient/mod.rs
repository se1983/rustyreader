use ureq;

pub fn create_url(subreddit: &str, optional_args: Option<String>) -> String{
    let args = optional_args.unwrap_or(
        String::from(""));
    format!("https://www.reddit.com/r/{}.json?{}", subreddit, args)
}

pub fn gather_site(url: String) -> String {
    let resp = ureq::get(&url).call();
    if !resp.ok(){
        panic!("{:?}", resp.into_string())
    }
    // ToDo logging
    println!("{}", resp.status());

    let text = resp.into_string().unwrap_or_else(|error|{
        // ToDo Handle error better -- don't panic!
        panic!("{:?}", error)
    });

    // ToDo logging
    println!("{}", text);
    text
}