use clap::{App, Arg, SubCommand};

pub fn cli<'a, 'b>() -> App<'a, 'b> {
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
