use helpers::convert_ret;
use serde::{de::DeserializeOwned, Serialize};
use wasmer::{ExportError, Instance, Memory, NativeFunc};

use crate::{convert_slice, CallWithTuple, FuncInfo, START_WASM_MEMORY};

/*
@TODO Clean / revamp the API
 */

fn extract_result<Output>(mem: &Memory, ptr: usize) -> Result<Output, Box<dyn std::error::Error>>
where
    Output: DeserializeOwned,
{
    let view = mem.view::<u8>();
    let len_bytes = (0..4)
        .map(|idx| {
            view.get(1 + idx)
                .map(|c| c.get())
                .ok_or(format!("unable to get new length part {}", idx))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("Cannot get result length")
        .try_into()
        .unwrap();
    let len = u32::from_ne_bytes(len_bytes);
    let buf: Vec<u8> = view[ptr..(ptr + len as usize)]
        .iter()
        .map(|c| c.get())
        .collect();
    Ok(convert_slice::<Output>(&buf))
}

/// Get a function typed and named with `FuncType` in the [`Instance`]
///
/// - The first result is errors when getting the [`NativeFunc`] from the instance exports.
/// - The second result is errors at runtime when calling the function
///
/// *NOTE: This function require a tuple of args (with at least 1 arg and at most 20)
/// that are [`Serialize`] and Output which is [`DeserializeOwned`]*
pub fn get_func_complex_with_args<FuncType>(
    instance: &Instance,
) -> Result<
    impl Fn(FuncType::Inputs) -> Result<FuncType::Output, Box<dyn std::error::Error>> + '_,
    ExportError,
>
where
    FuncType: FuncInfo,
    FuncType::Inputs: Serialize,
    FuncType::Output: Clone,
    FuncType::Output: DeserializeOwned,
{
    let mem = instance.exports.get_memory("memory")?;

    (instance.exports.get_native_function(FuncType::FUNC_ID)
        as Result<NativeFunc<(i32, i32), i32>, ExportError>)
        .map(|native_func| {
            move |args| {
                // Inject params
                let serialized_data = convert_ret(args);
                let len = serialized_data.len();
                mem.view()[START_WASM_MEMORY..len + START_WASM_MEMORY]
                    .iter()
                    .zip(serialized_data.iter())
                    .for_each(|(cell, byte)| cell.set(*byte));

                let ptr =
                    native_func.call_with_tuple((START_WASM_MEMORY as _, len as i32))? as usize;

                // Extract result
                extract_result(mem, ptr)
            }
        })
}

/// Get a function typed and named with `FuncType` in the [`Instance`]
///
/// - The first result is errors when getting the [`NativeFunc`] from the instance exports.
/// - The second result is errors at runtime when calling the function
///
/// *NOTE: This function require a Output which is [`DeserializeOwned`]*
pub fn get_func_complex_without_args<FuncType>(
    instance: &Instance,
) -> Result<impl Fn() -> Result<FuncType::Output, Box<dyn std::error::Error>> + '_, ExportError>
where
    FuncType: FuncInfo,
    FuncType::Inputs: Serialize,
    FuncType::Output: Clone,
    FuncType::Output: DeserializeOwned,
{
    let mem = instance.exports.get_memory("memory")?;

    (instance.exports.get_native_function(FuncType::FUNC_ID)
        as Result<NativeFunc<(), i32>, ExportError>)
        .map(|native_func| {
            move || {
                let ptr = native_func.call()? as usize;

                // Extract result
                extract_result(mem, ptr)
            }
        })
}
