use reqwest::blocking::RequestBuilder;

use crate::{
    enums::method::Method,
    models::{request::Request, response::Response},
};

use super::{http_response_builder::HttpResponseBuilder, request_executor::RequestExecutor};

type HttpClientError = reqwest::Error;
type HttpClientResponse = reqwest::blocking::Response;
type HttpClient = reqwest::blocking::Client;

pub struct HttpRequestExecutor {
    client: HttpClient,
    response_builder: HttpResponseBuilder,
}

impl RequestExecutor for HttpRequestExecutor {
    fn execute(&self, request: &Request) -> Response {
        match self.execute_request(request) {
            Ok(response) => response,
            Err(_error) => todo!(),
        }
    }
}

impl HttpRequestExecutor {
    pub fn new() -> HttpRequestExecutor {
        HttpRequestExecutor {
            client: HttpClient::new(),
            response_builder: HttpResponseBuilder::new(),
        }
    }

    fn execute_request(&self, request: &Request) -> Result<Response, HttpClientError> {
        let mut request_builder = match request.method {
            Method::Get => self.get(request),
            Method::Post => self.post(request),
            Method::Put => todo!(),
            Method::Patch => todo!(),
            Method::Update => todo!(),
            Method::Delete => todo!(),
        };

        request_builder = self.insert_headers(&request, request_builder);

        request_builder
            .send()
            .map(|r| self.response_builder.build(&r))
    }

    fn get(&self, request: &Request) -> RequestBuilder {
        self.client.get(&request.uri)
    }

    fn post(&self, request: &Request) -> RequestBuilder {
        self.client.post(&request.uri).body(request.body.clone())
    }

    fn insert_headers(
        &self,
        from: &Request,
        to: reqwest::blocking::RequestBuilder,
    ) -> reqwest::blocking::RequestBuilder {
        let mut builder = to;

        for (name, header) in &from.headers {
            builder = builder.header(name, header.value());
        }

        builder
    }
}
