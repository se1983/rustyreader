# RustyReader

**NOTE!** This is *just a learning-project* for rust to have some fun coding. The code at the current state is not meant to run in realitity and will change rappidly! 


```rust
for listing in RedditClient::new("/r/all").limit(3) {
    println!("\n\n'{}' has {} comments.\n\t\thttps://reddit.com{}\n\t ",
             listing.link.data.children[0].data.title,
             listing.comments.data.children.len(),
             listing.link.data.children[0].data.permalink,
    );
};

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

