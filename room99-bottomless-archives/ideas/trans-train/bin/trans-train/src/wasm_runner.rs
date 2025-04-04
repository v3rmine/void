use std::{collections::HashMap, fs::File};

use clap::crate_version;
use semver::{Version, VersionReq};
use wasmtime::{
    component::{Component, Linker},
    Config, Engine, Store,
};

use crate::transformers::{apis::Apis, permissions::Permission};

mod bindings {
    wasmtime::component::bindgen!({
        world: "transformer",
        path: "../../wit"
    });
}

#[derive(Debug)]
pub struct TransformerState {
    pub apis: Apis,
    pub allowed_permissions: Vec<Permission>,
    pub opened_files: HashMap<String, File>,
}

/*// inputs permission
fn close_file(&mut self, handle: bindings::FileHandle) -> wasmtime::Result<()> {
    self.require_permissions(&[
        Permission::Inputs.into(),
        Permission::Outputs.into()
    ], false);

    let (FileHandle::Read(id) | FileHandle::ReadAndWrite(id)) = handle;
    // Take ownership of returned value and drop it if the file is opened
    if self.opened_files.remove(&id).is_none() {
        // TODO: Return error file is already closed
    }

    Ok(())
}
fn open_input(&mut self, path: String) -> wasmtime::Result<bindings::FileHandle> {
    self.require_permissions(&[Permission::Inputs.into()], true);

    let id = Uuid::new_v4().to_string();

    // TODO: Map FS error to WASI error
    self.opened_files.insert(id.clone(), File::options().read(true).open(path).unwrap());

    Ok(bindings::FileHandle::Read(id))
}
fn read(&mut self, handle: bindings::FileHandle) -> wasmtime::Result<(Vec<u8>, u32)> {
    self.require_permissions(&[Permission::Inputs.into(), Permission::Outputs.into()], false);

    let (FileHandle::Read(id) | FileHandle::ReadAndWrite(id)) = handle;
    let mut buf = Vec::new();
    let mut len = 0u32;

    // TODO: Throw error on IO error or if no file
    if let Some(file) = self.opened_files.get_mut(&id) {
        len = file.read(&mut buf).unwrap() as u32;
    }
    Ok((buf, len))
}
// outputs permission
fn open_output(&mut self, path: String) -> wasmtime::Result<bindings::FileHandle> {
    self.require_permissions(&[Permission::Outputs.into()], true);

    let id = Uuid::new_v4().to_string();

    // TODO: Map FS error to WASI error
    self.opened_files.insert(id.clone(), File::options().read(true).write(true).create(true).open(path).unwrap());

    Ok(bindings::FileHandle::ReadAndWrite(id))
}
fn write(&mut self, handle: FileHandle, content: Vec<u8>) -> wasmtime::Result<u32> {
    self.require_permissions(&[Permission::Outputs.into()], true);

    let (FileHandle::Read(id) | FileHandle::ReadAndWrite(id)) = handle;
    let mut len = 0u32;

    // TODO: Throw error on IO error or if no file
    if let Some(file) = self.opened_files.get_mut(&id) {
        len = file.write(&content).unwrap() as u32;
    }

    Ok(len)
}
fn write_flush(&mut self, handle: FileHandle) -> wasmtime::Result<()> {
    self.require_permissions(&[Permission::Outputs.into()], true);

    let (FileHandle::Read(id) | FileHandle::ReadAndWrite(id)) = handle;

    // TODO: Throw error on IO error or if no file
    if let Some(file) = self.opened_files.get_mut(&id) {
        file.flush().unwrap();
    }

    Ok(())
}
// config permission
fn get_config(&mut self, key: String) -> wasmtime::Result<String> {
    self.require_permissions(&[Permission::Config.into()], true);
    // TODO: Get config value
    Ok(key)
}
fn get_whole_transformer_config(&mut self) -> wasmtime::Result<String> {
    self.require_permissions(&[Permission::AllConfig.into()], true);
    Ok(String::new())
}*/

#[derive(Debug, Default)]
pub struct GuestState {
    pub imports: Vec<TransformerState>,
}

pub struct WasiTransformer {
    bindings: bindings::TransformerWorld,
    pub name: String,
    pub version: String,
    pub supported_version: VersionReq,
    pub permissions: Vec<String>,
}

pub struct WasiTransformerRuntime {
    store: Store<GuestState>,
    engine: Engine,
    crate_version: Version,
    pub transformers: HashMap<String, WasiTransformer>,
}

impl Default for WasiTransformerRuntime {
    fn default() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);

        let engine = wasmtime::Engine::new(&config).unwrap();
        let store = Store::new(&engine, GuestState::default());

        Self {
            engine,
            store,
            crate_version: Version::parse(crate_version!()).unwrap(),
            transformers: HashMap::new(),
        }
    }
}

impl WasiTransformerRuntime {
    #[tracing::instrument(skip(self))]
    pub fn register_transformer(
        &mut self,
        name: &str,
        transformer_path: &str,
        allowed_permissions: Vec<Permission>,
    ) -> eyre::Result<&mut Self> {
        let module_state = TransformerState {
            opened_files: HashMap::new(),
            allowed_permissions,
            apis: Apis::new(name)?,
        };
        let entry = self.store.data().imports.len();
        self.store.data_mut().imports.push(module_state);

        let mut linker = Linker::<GuestState>::new(&self.engine);
        Apis::link(entry, &mut linker)?;

        tracing::debug!("compile component");
        let component = Component::from_file(&self.engine, transformer_path).unwrap();

        let (bindings, _) =
            bindings::TransformerWorld::instantiate(&mut self.store, &component, &linker).unwrap();

        tracing::debug!("initialize component");
        bindings.transformer().call_init(&mut self.store, &[]).unwrap();

        let definition = bindings
            .transformer()
            .call_get_definition(&mut self.store)
            .unwrap();
        let version_req = VersionReq::parse(&definition.supported_version)?;
        if !version_req.matches(&self.crate_version) {
            tracing::error!("tranformer does not support crate version");
            return Err(eyre::eyre!("tranformer {name} does not support crate version"));
        }

        self.transformers.insert(
            name.to_string(),
            WasiTransformer {
                bindings,
                name: name.to_string(),
                version: definition.version,
                permissions: definition.required_permissions,
                supported_version: version_req,
            },
        );

        Ok(self)
    }

    #[tracing::instrument(skip(self))]
    pub fn call(&mut self, name: &str) -> eyre::Result<()> {
        self.transformers
            .get(name)
            .map(|t| {
                t.bindings.transformer().call_call(&mut self.store).unwrap();
            })
            .unwrap();
        Ok(())
    }
}
