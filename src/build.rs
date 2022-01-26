use colored::Colorize;
use minreq::get;
use std::{fs, process};
use url::Url;

static TEMPLATE: &str = include_str!("template.html");

pub fn build_react_app(file: &str) -> String {
    let is_url = Url::parse(&file).is_ok();

    let app = if is_url {
        let resp = match get(&*file).send() {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error fetching that URL: {}", e);
                process::exit(1);
            }
        };

        let app = match resp.as_str() {
            Ok(app) => app,
            Err(e) => {
                eprintln!("Error parsing response as string: {}", e);
                process::exit(1);
            }
        };

        String::from(app)
    } else {
        match fs::read_to_string(file) {
            Ok(app) => app,
            Err(e) => {
                eprintln!("Could not read file \"{}\": {}.", file.green(), e);
                process::exit(1);
            }
        }
    };

    TEMPLATE.replace("// APP", &app)
}
