# RustyReader

**NOTE!** This is just a project to learn rust and having some fun coding. The code at the current state is not meant to run in realitity and will change rappidly! Feel free to look into the code but I am not looking for contributers, improvement proposals are very welcome.

```rust
let subreddit = String::from("/r/programming/new");
let listings = RedditClient::new(subreddit).limit(5).get();
println!("{}", listings[0].comment.link.data.children[0].data.title);
// Implementing the Clipper chip cipher in Rust
```


## Goal
The idea  of the project is a configurable reddit-bot. 

It reads definitions and commands from yaml files and is able to load plugins to reply to specified commands in specified subreddits.

1. Read yaml files for definitions and commands (eg: subreddit, trigger, answer)
2. Use declaration to load plugins
3. Monitor data on defined subreddit
3. Answers on triggers with defined commands

## PoC
- !RR fortune  -> Answer with GNU/fortune
- !RR calc(2+7) -> Simple Calculator for reddit (.. yes I know it's silly :D)

