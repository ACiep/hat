extern crate clap;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

mod cli;
mod config;
mod parser;
pub mod request;

use config::Project;
use hyper::rt;
use request::Request;

fn main() {
    let options = cli::cli().get_matches();

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

            rt::run(rt::lazy(move || request.run()));
        }
    }
}