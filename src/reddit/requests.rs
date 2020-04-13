use std::collections::HashMap;
use std::time::{Duration, Instant};

use log::info;

pub struct Cacher {
    call: Box<dyn Fn(&str) -> String>,
    data: HashMap<String, (String, Instant)>,
    cache_time: Duration,
    count: i32,
}

impl Cacher {
    pub fn new<T: Fn(&str) -> String + 'static>(call: T) -> Cacher {
        Cacher {
            call: Box::new(call),
            data: HashMap::new(),
            cache_time: Duration::from_secs(100),
            count: 0,
        }
    }

    pub fn data(&mut self, url: &str) -> String {
        self.count += 1;
        match self.data.get(url) {
            Some((data, created)) if created.elapsed() < self.cache_time => {
                info!("Using cached Data from {} s ago.", created.elapsed().as_secs());
                data.clone()
            },
            _ => {
                info!("Start request {}", url);
                let now = Instant::now();
                let data = (self.call)(url);
                info!("Request done.");
                self.data.insert(String::from(url), (data.clone(), now));
                data
            }
        }
    }
}


pub fn get_request() -> impl Fn(&str) -> String {
    move |url| {
        log::info!("Requesting [GET] {}", url);
        let response = ureq::get(url).call();
        response.into_string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_calls_closure() {
        let mut c = Cacher::new(|x| x.to_string());
        assert_eq!(
            c.data("Foo"),
            "Foo"
        );

        assert_eq!(c.count, 1);
    }
}