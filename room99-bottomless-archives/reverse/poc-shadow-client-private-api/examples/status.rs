extern crate shadowtech_api;

use shadowtech_api::{Result, Shadow};

fn main() -> Result<()> {
  let resp = Shadow::empty()?.status();

  println!("{:?}", resp);

  Ok(())
}
