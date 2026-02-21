use esp_idf_svc::io::{Error, ErrorKind, Read};

pub fn get_next_byte(stream: &mut impl Read) -> Option<u8> {
    let mut buffer = [0u8];
    loop {
        log::debug!("get next byte in stream");
        match stream.read(&mut buffer) {
            Ok(_) => return Some(buffer[0]),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut => {
                    log::warn!("timed out waiting for a byte in stream");
                    return None;
                }
                e => {
                    log::error!("failed to read byte in stream (error kind: {:?})", e);
                    return None;
                }
            },
        };
    }
}
