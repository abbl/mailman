use crate::models::{request::Request, response::Response};

pub trait RequestExecutor {
    fn execute(&self, request: &Request) -> Response;
}
