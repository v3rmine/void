use std::io::{ErrorKind, Read};

use esp_idf_svc::hal::delay::FreeRtos;

pub fn get_next_byte(stream: &mut impl Read) -> Option<u8> {
    let mut buffer = [0u8];
    loop {
        match stream.read_exact(&mut buffer) {
            Ok(_) => return Some(buffer[0]),
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => FreeRtos::delay_ms(1),
                e => {
                    log::error!("failed to read byte in stream (error kind: {e})");
                    return None;
                }
            },
        };
    }
}
