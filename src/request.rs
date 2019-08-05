use std::collections::HashMap;

type Headers = HashMap<String, String>;

pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Headers,
    pub body: Option<String>,
}

impl Request {
    pub fn new(url: String, method: String, headers: Headers, body: Option<String>) -> Self {
        Self {
            url,
            method,
            headers,
            body,
        }
    }
}