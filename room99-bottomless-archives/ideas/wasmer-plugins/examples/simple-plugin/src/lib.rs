use simple_abi::GetVersionReq;
use wasm_plugins::plugin::complex_export;

#[complex_export(GetVersionReq)]
pub fn get_version_req() -> String {
    String::from("^0.1")
}
