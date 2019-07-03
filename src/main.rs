extern crate clap;
extern crate hyper;

mod parser;

use clap::{App, Arg};
use hyper::rt::{self, Future, Stream};
use hyper::{Client, Request, Response};
use std::io::{self, Write};

fn handle_response<T>(res: Response<T>) {
    println!("Status: {}", res.status());
}

fn main() {
    let matches = App::new("HTTP tester")
        .version("0.0")
        .about("Tool for testing HTTP requests")
        .arg(
            Arg::with_name("url")
                .help("URL to make request on")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("method")
                .help("HTTP request method")
                .short("m")
                .long("method")
                .takes_value(true)
                .possible_values(&["GET", "HEAD", "POST", "PUT", "DELETE", "PATCH"])
                .default_value("GET"),
        )
        .arg(
            Arg::with_name("body")
                .help("Text used to make HTTP request")
                .required(false)
                .short("b")
                .long("body")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("headers")
                .help("Add HTTP header to request")
                .required(false)
                .short("h")
                .long("header")
                .multiple(true)
                .takes_value(true),
        )
        .get_matches();

    let url = matches.value_of("url").expect("Pass URL value");
    let method = matches
        .value_of("method")
        .expect("Method is not correct")
        .to_string();
    let body = parser::body(matches.value_of("body"));
    let uri: hyper::Uri = url.parse().expect("URL is not correct");
    let headers = parser::headers(matches.values_of("headers").unwrap());

    rt::run(rt::lazy(move || {
        let client = Client::new();
        let mut req = Request::builder();
        for (key, value) in headers {
            req.header(key.as_str(), value.as_str());
        }
        let req = req
            .uri(uri)
            .method(method.as_str())
            .body(body)
            .expect("Failed building request");

        client.request(req).map(handle_response).map_err(|err| {
            eprintln!("Error: {}", err);
        })
    }));
}
