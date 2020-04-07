use ureq;
use std::collections::HashMap;
use std::time::{Instant, Duration};


pub struct Cacher<T> where T: Fn(&str) -> String {
    call: T,
    data: HashMap<String, (String, Instant)>,
    cache_time: Duration,
}

impl<T> Cacher<T> where T: Fn(&str) -> String {
    pub fn new(call: T) -> Cacher<T> {
        Cacher {
            call,
            data: HashMap::new(),
            cache_time: Duration::from_millis(10000),
        }
    }

    fn data(&mut self, url: &str) -> String {
        println!("Calling request {}", &url);

        match self.data.get(url) {
            Some((data, created)) if created.elapsed() < self.cache_time => data.clone(),
            _ => {
                let now = Instant::now();
                let data = (self.call)(url);
                self.data.insert(String::from(url), (data.clone(), now));
                data
            }
        }
    }
}