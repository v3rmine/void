use wasm_plugins::abi::InnerFunctionDetails;

#[derive(InnerFunctionDetails)]
pub struct GetVersionReq(pub fn() -> String);

#[derive(InnerFunctionDetails)]
pub struct GetPage(pub fn(String) -> String);
