use wasm_plugins::abi::InnerFunctionDetails;

#[derive(InnerFunctionDetails)]
pub struct GetVersionReq(pub fn() -> String);
