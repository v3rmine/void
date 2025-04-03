use esp_idf_hal::{prelude::Peripherals, gpio::PinDriver, delay::FreeRtos};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio1)?;
    loop {
        led.set_high()?;
        FreeRtos::delay_ms(1000);
        info!("Blink");
        led.set_low()?;
        FreeRtos::delay_ms(3000);
    }
}
