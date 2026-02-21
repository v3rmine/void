use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::*, uart};
use log::LevelFilter;

use crate::{constants::BAUDRATE, thermal::ThermalInterface};

mod constants;
mod helpers;
mod thermal;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::log::set_target_level("*", LevelFilter::Debug).unwrap();
    log::debug!("started logger");

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = uart::config::Config::default().baudrate(Hertz(BAUDRATE));

    let uart: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart1,
        pins.gpio1,
        pins.gpio0,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config,
    )
    .unwrap();
    log::debug!("created uart driver");

    let mut thermal = ThermalInterface::new(uart);
    thermal.begin(None);
    log::info!("started thermal interface");

    thermal.test_page();
    thermal.feed(2);
    // thermal.print("Hello World");
    thermal.feed(2);
    // if thermal.has_paper() {
    //     log::info!("printer has paper");
    // } else {
    //     log::info!("printer has no paper");
    // }
}
