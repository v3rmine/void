extern crate dotenv;
extern crate shadowtech_api;

use shadowtech_api::computer;
use shadowtech_api::Shadow;

use dotenv::var;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  dotenv::dotenv()?;

  let mut s = Shadow::new(
    var("EMAIL")?,
    var("PASSWORD")?,
    var("SESSION_UUID")?,
    var("SHADOW_UUID")?,
  )?;

  let resp = s.start_vm()?;

  println!("{:#?}", resp);

  Ok(())
}
