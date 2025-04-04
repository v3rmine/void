//! Based on <https://github.com/bytecodealliance/cargo-component>

use bindings::transformer;
use wit_transformer_utils::*;
use wit_log as log;

struct HelloWorldTransformer;

impl transformer::Transformer for HelloWorldTransformer {
    fn init(_permissions: Vec<transformer::Permissions>) {
        let _ = log::set_boxed_logger(Box::new(log::WitLog::new()));
        log::set_max_level(log::LevelFilter::Trace);
    }

    fn get_definition() -> transformer::TransformerDefinition {
        transformer::TransformerDefinition {
            name: "hello".to_string(),
            version: "0.1.0".to_string(),
            supported_version: "^0.1.0".to_string(),
            required_permissions: vec![],
        }
    }

    fn call() {
        log::info!("I WORK!");
    }
}

impl_transformer!(HelloWorldTransformer);
