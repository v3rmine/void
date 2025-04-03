use host_func_abi::GetVersionReq;
use wasm_plugins::plugin::complex_export;

#[complex_export(GetVersionReq)]
pub fn get_version_req() -> String {
    #[allow(unsafe_code)]
    unsafe {
        get_page(32);
    }
    String::from("^0.1")
}

extern "C" {
    fn get_page(url: i32) -> i32;
}
