extern crate dotenv;
extern crate shadowtech_api;

use dotenv::var;

use shadowtech_api::log;
use shadowtech_api::Shadow;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv()?;

  let mut s = Shadow::new(
    var("EMAIL")?,
    var("PASSWORD")?,
    var("SESSION_UUID")?,
    var("SHADOW_UUID")?,
  )?;

  let resp = s.log(log::LogQuery {
    version: 1,
    name: "launcher.status",
    timestamp: std::time::Instant::now().elapsed().as_millis() as u64,
    privacy: "PUBLIC",
    metadata: log::LogQueryMetadata {
      version: "4.12.2",
      packager_version: "5.0.402",
      launcher_version: "4.12.2",
      renderer_version: "2.17.21",
      user_environment: "preprod",
      os_family: "Mac",
      os_version: "17.7.0",
      session: &s.session_uuid,
      user_uuid: "",
      main_display: "Color LCD",
      main_display_resolution: "1440x900",
      cpu_model: "Core i5-2557M",
      cpu_cores: "4",
      cpu_frequency: "1.70",
      total_memory: "4",
      graphic_card: "Intel HD Graphics 3000",
    },
    values: log::LogQueryValues {
      status: "HEARTBEAT",
    },
  });

  println!("{:?}", resp);

  Ok(())
}
