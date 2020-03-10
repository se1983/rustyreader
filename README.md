# RustyReader 
Just a rust learning project. Working with data and the public API from reddit

## Features

- HTTP client implementated in ureq
- Marshaling JSON with serde
- Reading Redditpage and Comments

## TODOs
- unify webclient API -- add Simple entry point
```rust
let subreddit = String::from("/r/all/new");
let listings = RedditClient::new(subreddit).limit(15).get();
println!("{}", listings[0].comment.link.data.children[0].data.title);
// Irish Times: Suspect opts for legal defense in MH17 case trial | KyivPost
```
- Analyze data
- Auth
- POSTing data

