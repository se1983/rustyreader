
Just a rust learning project. Working with data and API from reddit

Features
• HTTP client implementated in ureq
• Marshaling JSON with serde
• Redditpage
• Comments

TODOs
• unify webclient API -- add Simple entry point

```rust
redditclient::get_data("/r/all/new").limit(15)
```

• Analyze data
• Auth
• POSTing data

