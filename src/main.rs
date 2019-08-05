extern crate clap;
extern crate hyper;

mod parser;
mod request;

use clap::{App, Arg};
use hyper::rt::{self, Future};
use hyper::{Client, Request as HyperRequest, Response};
use request::Request;

fn handle_response<T>(res: Response<T>) {
    println!("Status: {}", res.status());
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("HTTP API tester")
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
}

fn main() {
    let options = cli().get_matches();

    let url = options.value_of("url").expect("Pass URL value").to_string();
    let method = options
        .value_of("method")
        .expect("Method is not correct")
        .to_string();
    let body = options.value_of("body");
    let headers = parser::headers(options.values_of("headers"));
    let request = Request::new(url, method, headers, body.map(|b| b.to_string()));

    rt::run(rt::lazy(move || {
        let client = Client::new();
        let mut req = HyperRequest::builder();
        for (key, value) in request.headers {
            req.header(key.as_str(), value.as_str());
        }
        let req = req
            .uri(&request.url)
            .method(request.method.as_str())
            .body(parser::body(request.body))
            .expect("Failed building request");

        client.request(req).map(handle_response).map_err(|err| {
            eprintln!("Error: {}", err);
        })
    }));
}