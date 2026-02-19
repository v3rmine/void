use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::*, uart};

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

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let config = uart::config::Config::default().baudrate(Hertz(BAUDRATE));

    let uart: uart::UartDriver = uart::UartDriver::new(
        peripherals.uart0,
        pins.gpio0,
        pins.gpio1,
        Option::<AnyIOPin>::None,
        Option::<AnyIOPin>::None,
        &config,
    )
    .unwrap();

    let mut thermal = ThermalInterface::new(uart);
    thermal.begin(None);
}
