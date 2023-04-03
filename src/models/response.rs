use std::collections::HashMap;

use super::header::Header;

pub struct Response {
    pub headers: HashMap<String, Header>
}
