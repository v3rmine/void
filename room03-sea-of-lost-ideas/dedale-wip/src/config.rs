use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    app: String,
    build: BuildConfig,
    http_service: HttpServiceConfig,
    vm: Vec<VmConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpServiceConfig {
    internal_port: u16,
    auto_stop_machines: bool,
    auto_start_machines: bool,
    min_machines_running: u16,
    processes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VmSize(String);
#[derive(Debug, Serialize, Deserialize)]
pub struct VmConfig {
    size: VmSize,
}
