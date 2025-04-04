use crate::wasm_runner::GuestState;

wasmtime::component::bindgen!({
    path: "../../wit/log.wit",
});

#[derive(Debug)]
pub(super) struct LogApi {
    module_name: String,
}

impl LogApi {
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
        }
    }

    pub fn link(
        id: usize,
        linker: &mut wasmtime::component::Linker<GuestState>,
    ) -> eyre::Result<()> {
        log::add_to_linker(linker, move |s| &mut s.imports[id].apis.log).map_err(|e| eyre::eyre!(e))
    }
}

impl log::Host for LogApi {
    fn trace(&mut self, msg: String) -> wasmtime::Result<()> {
        tracing::trace!("{} - {msg}", self.module_name);
        Ok(())
    }
    fn debug(&mut self, msg: String) -> wasmtime::Result<()> {
        tracing::debug!("{} - {msg}", self.module_name);
        Ok(())
    }
    fn info(&mut self, msg: String) -> wasmtime::Result<()> {
        tracing::info!("{} - {msg}", self.module_name);
        Ok(())
    }
    fn warn(&mut self, msg: String) -> wasmtime::Result<()> {
        tracing::warn!("{} - {msg}", self.module_name);
        Ok(())
    }
    fn error(&mut self, msg: String) -> wasmtime::Result<()> {
        tracing::error!("{} - {msg}", self.module_name);
        Ok(())
    }
}
