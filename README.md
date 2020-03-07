# RustyReader 
Just a rust learning project. Working with data and the public API from reddit

## Features

- HTTP client implementated in ureq
- Marshaling JSON with serde
- Reading Redditpage and Comments

## TODOs
- unify webclient API -- add Simple entry point
```rust
RedditClient::get_data("/r/all/new").limit(15)
```
- Analyze data
- Auth
- POSTing data

