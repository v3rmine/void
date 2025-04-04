pub use rand::{thread_rng, Rng};
use base64::{encode, decode};

type Error = Box<dyn std::error::Error>;

pub fn sym_enc(msg: String) -> Result<(String, String, String), Error> {
    let rk = thread_rng().gen::<[u8; 16]>();
    let random = thread_rng().gen::<[u8; 16]>();

    let ciphertext = openssl::symm::encrypt(
        openssl::symm::Cipher::aes_128_cbc(),
        &rk,
        Some(&random),
        msg.as_bytes(),
    )
        .map_err(|e| e.to_string())?;

    Ok((
        encode(&rk.to_vec()),
        encode(&random.to_vec()),
        encode(&ciphertext),
    ))
}
pub fn sym_enc_with_params(msg: String, rk: &[u8], random: &[u8]) -> Result<String, Error> {
    let ciphertext = openssl::symm::encrypt(
        openssl::symm::Cipher::aes_128_cbc(),
        rk,
        Some(random),
        msg.as_bytes(),
    ).map_err(|e| e.to_string())?;

    Ok(encode(&ciphertext))
}
pub fn sym_dec(key: String, iv: String, encrypted: String) -> Result<Vec<u8>, Error> {
    Ok(openssl::symm::decrypt(
        openssl::symm::Cipher::aes_128_cbc(),
        &decode(&key)?,
        Some(&decode(&iv)?),
        &decode(&encrypted)?,
    )
        .map_err(|e| e.to_string())?)
}
