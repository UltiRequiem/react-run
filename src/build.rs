use minreq::get;
use std::{fs, path, sync};
use url::Url;

use swc::{
    self,
    config::{JscConfig, Options},
};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
};

static TEMPLATE: &str = include_str!("template.html");

fn compile_jsx(file_path: &str) -> String {
    let cm = sync::Arc::<SourceMap>::default();

    let fm = cm
        .load_file(path::Path::new(file_path))
        .expect("failed to load file");

    let handler = sync::Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));

    let c = swc::Compiler::new(cm.clone());

    let output = c
        .process_js_file(
            fm,
            &handler,
            &Options {
                config: swc::config::Config {
                    jsc: JscConfig {
                        syntax: Some(swc_ecma_parser::Syntax::Typescript(
                            swc_ecma_parser::TsConfig {
                                tsx: true,
                                ..Default::default()
                            },
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("failed to process file");

    output.code
}

fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub fn react_app(file: &str) -> String {
    /* let app = if is_valid_url(file) {
           get(file)
               .send()
               .unwrap_or_else(|e| panic!("Error fetching \"{}\": {:?}", file, e))
               .as_str()
               .unwrap_or_else(|e| panic!("Error parsing response as string: {:?}", e))
               .to_owned()
       } else {
           fs::read_to_string(file)
               .unwrap_or_else(|e| panic!("Could not read file \"{}\": {:?}.", file, e))
       };
    */

    let app = compile_jsx(file);
    TEMPLATE.replace("// APP", &app)
}
