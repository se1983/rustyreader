use std::collections::HashMap;
use std::time::{Instant, Duration};


pub struct Cacher {
    call: Box<dyn Fn(&str) -> String>,
    data: HashMap<String, (String, Instant)>,
    cache_time: Duration,
}

impl Cacher {
    pub fn new<T: Fn(&str) -> String + 'static>(call: T) -> Cacher {
        Cacher {
            call: Box::new(call),
            data: HashMap::new(),
            cache_time: Duration::from_secs(100),
        }
    }

    pub(crate) fn data(&mut self, url: &str) -> String {
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

