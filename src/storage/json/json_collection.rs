use serde::{Deserialize, Serialize};

use crate::models::{collection::Collection, header::Header, request::Request};

use super::{json_method::JsonMethod, json_request::JsonRequest};

#[derive(Serialize, Deserialize)]
pub struct JsonCollection {
    pub name: String,
    pub requests: Vec<JsonRequest>,
}

impl JsonCollection {
    pub fn new(collection: &Collection) -> JsonCollection {
        JsonCollection {
            name: collection.name.clone(),
            requests: JsonCollection::to_json_requests(&collection.requests),
        }
    }

    pub fn to_collection(&self) -> Collection {
        let mut collection = Collection::new(&self.name);

        for json_request in &self.requests {
            let mut request = Request::new(
                JsonMethod::to_method(&json_request.method),
                json_request.uri.clone(),
            );

            request.add_body_binary(&json_request.body);

            for (json_name, json_header) in &json_request.headers {
                let mut header = Header::new(&json_header.value);

                if json_header.is_enabled {
                    header.enable();
                } else {
                    header.disable();
                }

                request.add_header(&json_name, header);
            }

            collection.add_request(request);
        }

        collection
    }

    fn to_json_requests(requests: &Vec<Request>) -> Vec<JsonRequest> {
        requests.iter().map(|v| JsonRequest::new(v)).collect()
    }
}
