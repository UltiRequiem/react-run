use minreq::get;
use std::fs;
use url::Url;

static TEMPLATE: &str = include_str!("template.html");

fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub async fn react_app(file: &str) -> String {
    let app = if is_valid_url(file) {
        let resp = get(&*file)
            .send()
            .unwrap_or_else(|e| panic!("Error fetching \"{}\": {:?}", file, e));

        let app = resp
            .as_str()
            .unwrap_or_else(|e| panic!("Error parsing response as string: {:?}", e));

        app.to_owned()
    } else {
        fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("Could not read file \"{}\": {:?}.", file, e))
    };

    TEMPLATE.replace("// APP", &app)
}