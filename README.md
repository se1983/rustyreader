# RustyReader 
Just a rust learning project. Working with data and the public API from reddit

```rust
let subreddit = String::from("/r/programming/new");
let listings = RedditClient::new(subreddit).limit(5).get();
println!("{}", listings[0].comment.link.data.children[0].data.title);
// Implementing the Clipper chip cipher in Rust
```


The idea of the project is a configurable reddit-bot. It reads definitions and commands from yaml files and is able to load plugins to reply to specified commands in specified subreddits.

## Features

- HTTP client implementated in ureq
- Marshaling JSON with serde
- Reading Redditpage and Comments

## TODOs
- unify webclient API -- add Simple entry point

- Analyze data
- Auth
- POSTing data

