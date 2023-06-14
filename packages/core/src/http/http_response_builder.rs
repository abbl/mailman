use std::collections::HashMap;

use crate::models::header::Header;
use crate::models::response::Response;

pub struct HttpResponseBuilder {}

impl HttpResponseBuilder {
    pub fn new() -> HttpResponseBuilder {
        HttpResponseBuilder {}
    }

    pub fn build(&self, response: &reqwest::blocking::Response) -> Response {
        Response {
            headers: self.convert_headers(response)
        }
    }

    fn convert_headers(&self, response: &reqwest::blocking::Response) -> HashMap<String, Header> {
        let mut headers: HashMap<String, Header> = HashMap::new();

        for (header_name, header_value) in response.headers() {
            let value = self.stringify_header(header_value);

            headers.insert(header_name.to_string(), Header::new(&value));
        }

        headers
    }

    fn stringify_header<'a>(&self, header_value: &reqwest::header::HeaderValue) -> String {
        match header_value.to_str() {
            Ok(value) => value.to_string(),
            Err(_err) => String::from("[Err] Could not stringify the header."),
        }
    }
} 
