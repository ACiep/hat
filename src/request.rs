use crate::parser;
use hyper::rt::{Future, Stream};
use hyper::{Client, Request as HyperRequest};
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

    pub fn run(&self) -> impl Future<Item = (), Error = ()> {
        let client = Client::new();
        let mut req = HyperRequest::builder();

        for (key, value) in self.headers.clone() {
            req.header(key.as_str(), value.as_str());
        }

        let req = req
            .uri(&self.url)
            .method(self.method.as_str())
            .body(parser::body(self.body.clone()))
            .expect("Failed building request");

        client
            .request(req)
            .and_then(|res| {
                println!("Status: {}", res.status());
                res.into_body().concat2()
            })
            .from_err::<hyper::Error>()
            .and_then(|chunk| Ok(String::from_utf8(chunk.into_bytes().to_vec())))
            .map(|body| {
                println!("Body:\n{}", body.unwrap());
            })
            .map_err(|err| eprintln!("Error: {}", err))
    }
}

impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
{}
URL: {}
Method: {}
",
            self.name, self.url, self.method
        )
    }
}