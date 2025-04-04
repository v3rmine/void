use crate::wasm_runner::GuestState;

use self::{log::LogApi, sys::SysApi};

mod log;
mod sys;

#[derive(Debug)]
pub struct Apis {
    sys: SysApi,
    log: LogApi,
}

impl Apis {
    pub fn new(module_name: &str) -> eyre::Result<Self> {
        Ok(Self {
            log: LogApi::new(module_name),
            sys: SysApi::default(),
        })
    }

    pub fn link(
        id: usize,
        linker: &mut wasmtime::component::Linker<GuestState>,
    ) -> eyre::Result<()> {
        sys::SysApi::link(id, linker)?;
        log::LogApi::link(id, linker)?;
        Ok(())
    }
}
