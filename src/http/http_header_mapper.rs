use crate::{enums::headers::ContentType, models::header::Header};

pub struct HttpHeaderMapper {}

impl HttpHeaderMapper {
    pub fn content_type(content_type: ContentType) -> (String, Header) {
        let value = match content_type {
            ContentType::Text => "text/plain",
            ContentType::Json => "application/json",
            ContentType::FormData => "multipart/form-data",
        };

        ("Content-Type".to_owned(), Header::new(value))
    }
}
