use esp_idf_svc::io::{Error, Read, ReadExactError};

pub fn get_next_byte(stream: &mut impl Read) -> Option<u8> {
    let mut buffer = [0u8];
    loop {
        match stream.read_exact(&mut buffer) {
            Ok(_) => return Some(buffer[0]),
            Err(ReadExactError::UnexpectedEof) => {
                log::error!("failed to read byte in stream UnexpectedEof");
            }
            Err(ReadExactError::Other(e)) => match e.kind() {
                e => {
                    log::error!("failed to read byte in stream (error kind: {:?})", e);
                    return None;
                }
            },
        };
    }
}
