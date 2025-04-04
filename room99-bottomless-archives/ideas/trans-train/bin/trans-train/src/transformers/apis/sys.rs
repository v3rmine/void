use crate::wasm_runner::GuestState;

wasmtime::component::bindgen!({
    path: "../../wit/sys.wit",
});

use uuid::Uuid;

#[derive(Debug, Default)]
pub(super) struct SysApi;

impl SysApi {
    pub fn link(
        id: usize,
        linker: &mut wasmtime::component::Linker<GuestState>,
    ) -> eyre::Result<()> {
        sys::add_to_linker(linker, move |s| &mut s.imports[id].apis.sys)
            .map_err(|e| eyre::eyre!("failed to link sys to WASM runtime {:?}", e))
    }
}

impl sys::Host for SysApi {
    fn rand_u64(&mut self) -> wasmtime::Result<u64> {
        Ok(rand::random())
    }
    fn uuid4(&mut self) -> wasmtime::Result<String> {
        Ok(Uuid::new_v4().to_string())
    }
}
