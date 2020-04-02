use ureq;


struct Cacher<T> where T: Fn(&str) -> String {
    operation: T,
    data: Option<String>
}

impl<T> Cacher<T> where T: Fn(&str) -> String{
    fn new(operation: T) -> Self<T> {
        Self {
            operation,
            data: None
        }
    }


    fn data(&mut self, url: &str, query: &str) -> String {
        match &self.data {
            Some(d) => d.clone(),
            None    => {
                let response = (self.operation)(&url, &query);
                let d = response.into_string().unwrap();
                self.data  = Some(d);
                d.clone()
            }
        }
    }
}



