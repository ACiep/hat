extern crate clap;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod config;
mod parser;
pub mod request;

use clap::{App, Arg, SubCommand};
use config::Project;
use hyper::rt::{self, Future, Stream};
use hyper::{Client, Request as HyperRequest};
use request::Request;

fn cli<'a, 'b>() -> App<'a, 'b> {
    App::new("HTTP API tester")
        .version("0.0")
        .about("Tool for testing HTTP requests")
        .arg(
            Arg::with_name("url")
                .help("URL to make request on")
                // TODO
                // .required_unless("list")
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
        .arg(
            Arg::with_name("request to save")
                .help("Save this request")
                .required(false)
                .short("s")
                .long("save")
                .takes_value(true),
        )
        .subcommand(SubCommand::with_name("init"))
        .subcommand(SubCommand::with_name("list"))
}

fn run_request(request: Request) {
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

        client
            .request(req)
            .and_then(|res| {
                println!("Status: {}", res.status());
                res.into_body().concat2()
            })
            .from_err::<hyper::Error>()
            .and_then(|chunk| Ok(String::from_utf8(chunk.into_bytes().to_vec())))
            .map(|body| {
                println!("Body: {}", body.unwrap());
            })
            .map_err(|err| eprintln!("Error: {}", err))
    }));
}

fn main() {
    let options = cli().get_matches();

    match options.subcommand() {
        ("init", Some(_)) => match Project::create() {
            Err(e) => eprintln!("Error occured. {}", e),
            Ok(_) => println!(
                ".hat.toml created. You can now save requests and much more in the future."
            ),
        },
        ("list", Some(_)) => {
            let project = Project::get();
            println!("{}", project);
        }
        _ => {
            let name = options.value_of("request to save");
            let url = options.value_of("url").expect("Pass URL value").to_string();
            let method = options
                .value_of("method")
                .expect("Method is not correct")
                .to_string();
            let body = options.value_of("body");
            let headers = parser::headers(options.values_of("headers"));

            let request = Request::new(
                url,
                method,
                body.map(|b| b.to_string()),
                headers,
                name.unwrap_or("").to_string(),
            );

            match name {
                None => {}
                Some(_) => {
                    let mut project = Project::get();
                    project.save_request(request.clone());
                }
            }

            run_request(request);
        }
    }
}