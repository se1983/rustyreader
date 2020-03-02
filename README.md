# rustyreader

Just a rust learning project.  Working with data and API from reddit


## Features
- HTTP client implementated in ureq 
- Marshaling JSON with serde
    - Redditpage
    - Comments


## TODOs
- add consteuczltor for marshalled data with using traits  with serde deserializer
```rust
// Something like this
let deserialized_data = RedditPage::new(&data)
```
- Analyze data
- Auth
- POSTing data




