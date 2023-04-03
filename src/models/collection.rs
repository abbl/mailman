use super::request::Request;

#[derive(Clone)]
pub struct Collection {
    pub name: String,
    pub requests: Vec<Request>,
}

impl Collection {
    pub fn new(name: &str) -> Collection {
        Collection {
            name: name.to_string(),
            requests: Vec::new(),
        }
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub fn deduplicate_name(&mut self) -> () {
        self.name = self.name.to_string() + "_copy";
    }
}
