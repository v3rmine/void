use std::fs;

use helpers::FuncInfo;
use wasmer::{
    imports, ExportError, FromToNativeWasmType, Instance, Module, NativeFunc, RuntimeError, Store,
    WasmTypeList,
};

use crate::CallWithTuple;

/// Get a function typed and named with `FuncType` in the [`Instance`]
///
/// - The first result is errors when getting the [`NativeFunc`] from the instance exports.
/// - The second result is errors at runtime when calling the function
///
/// *NOTE: This function only support standard WASM types see [`FromToNativeWasmType`]*
pub fn get_func_simple<FuncType>(
    instance: &Instance,
) -> Result<impl Fn(FuncType::Inputs) -> Result<FuncType::Output, RuntimeError>, ExportError>
where
    FuncType: FuncInfo,
    NativeFunc<FuncType::Inputs, FuncType::Output>:
        CallWithTuple<FuncType::Inputs, FuncType::Output>,
    FuncType::Inputs: WasmTypeList,
    FuncType::Output: FromToNativeWasmType,
{
    (instance.exports.get_native_function(FuncType::FUNC_ID)
        as Result<NativeFunc<FuncType::Inputs, FuncType::Output>, ExportError>)
        .map(|native_func| move |args| CallWithTuple::call_with_tuple(&native_func, args))
}

/// Create a basic [`Instance`] from a path to a wasm file
pub fn instanciate_simple(path: &str) -> Result<Instance, Box<dyn std::error::Error>> {
    let module_wat = fs::read(path)?;
    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    let import_object = imports! {};
    Ok(Instance::new(&module, &import_object)?)
}
