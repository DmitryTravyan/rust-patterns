extern crate clap;

use {
    clap::{Arg, App, SubCommand},
    reqwest::{Client, Response, Error},
    futures::Future,
};

// Как и все остальные примеры, этот так же основан на абстрактной web задаче
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("On stack dynamic dispatch")
        .version("0.1.0")
        .subcommands(vec![
            SubCommand::with_name("get")
                .arg(Arg::with_name("url")
                    .long("url")
                    .takes_value(true)
                    .help("http or https url for request")
                ),
            SubCommand::with_name("post")
                .arg(Arg::with_name("url")
                    .long("url")
                    .takes_value(true)
                    .help("http or https url for request")
                ),
        ])
        .get_matches();

    let client = Client::new();

    // Эти значения должны жить дольше чем переменная readable, поэтому мы
    // должны объявить их первыми:
    let (mut get, mut post);

    let _response: &dyn Future<Output=Result<Response, Error>> = match &matches.subcommand_name() {
        Some("get") => {
            let url = &matches
                .subcommand_matches("get")
                .unwrap()
                .value_of("url")
                .unwrap()
                .to_string();
            get = client.get(url)
                .send();
            &mut get
        }
        Some("post") => {
            let url = &matches
                .subcommand_matches("post")
                .unwrap()
                .value_of("url")
                .unwrap()
                .to_string();
            post = client.get(url)
                .body("body")
                .send();
            &mut post
        }
        _ => panic!("Error")
    };

    Ok(())
}
