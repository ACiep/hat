use std::collections::HashMap;

type Headers = HashMap<String, String>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: String,
    pub body: Option<String>,
    pub headers: Headers,
}

impl Request {
    pub fn new(
        url: String,
        method: String,
        body: Option<String>,
        headers: Headers,
        name: String,
    ) -> Self {
        Self {
            name,
            url,
            method,
            headers,
            body,
        }
    }
}