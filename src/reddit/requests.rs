use std::collections::HashMap;
use std::time::{Instant, Duration};
use log::info;


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

    pub fn data(&mut self, url: &str) -> String {
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