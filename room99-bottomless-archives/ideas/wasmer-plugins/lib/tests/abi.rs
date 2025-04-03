#[cfg(not(any(feature = "wasmer-rt", feature = "wasmer")))]
mod abi {
    use assay::assay;
    use wasm_plugins::abi::{FuncInfo, InnerFunctionDetails};

    #[derive(InnerFunctionDetails)]
    struct DefaultFuncId(pub fn() -> u32);

    #[derive(InnerFunctionDetails)]
    #[details(function_id = "custom")]
    struct CustomFuncId(pub fn() -> u32);

    #[assay]
    fn default_func_id() {
        assert_eq!(DefaultFuncId::FUNC_ID, "default_func_id");
    }

    #[assay]
    fn custom_func_id() {
        assert_eq!(CustomFuncId::FUNC_ID, "custom");
    }
}
