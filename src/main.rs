extern crate clap;

use clap::{App, Arg};

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
                .short("b")
                .long("body")
                .takes_value(true),
        )
        .get_matches();
}
