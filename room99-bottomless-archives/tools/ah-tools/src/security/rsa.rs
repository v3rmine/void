use serde::{Serialize, Deserialize};
use base64::{encode, decode};
use super::aes::{sym_dec, sym_enc};
use std::fs::File;
use std::io::Read;
use openssl::rsa::Rsa;

type Error = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tomb {
    pub aes_key: String,
    pub aes_iv: String,
    pub value: String,
}

pub fn public_encrypt(key: &openssl::rsa::Rsa<openssl::pkey::Public>, bytes: &[u8]) -> Result<Tomb, Error> {
    let mut sym_crypt = vec![0; key.size() as usize];
    let mut iv_crypt = vec![0; key.size() as usize];
    let (sym_key, iv, text) = sym_enc(String::from_utf8(bytes.to_vec())?)?;
    key.public_encrypt(
        &decode(&sym_key)?,
        &mut sym_crypt,
        openssl::rsa::Padding::PKCS1,
    )?;
    key.public_encrypt(&decode(&iv)?, &mut iv_crypt, openssl::rsa::Padding::PKCS1)?;
    Ok(Tomb {
        aes_key: encode(&sym_crypt),
        aes_iv: encode(&iv_crypt),
        value: text,
    })
}
pub fn public_decrypt(key: &Rsa<openssl::pkey::Public>, bytes: Tomb) -> Result<Tomb, Error> {
    let mut sym = vec![0; key.size() as usize];
    let mut iv = vec![0; key.size() as usize];
    let len_sym = key.public_decrypt(
        &decode(&bytes.aes_key)?,
        &mut sym,
        openssl::rsa::Padding::PKCS1,
    )?;
    let len_iv = key.public_decrypt(
        &decode(&bytes.aes_iv)?,
        &mut iv,
        openssl::rsa::Padding::PKCS1,
    )?;
    let sym = encode(&sym[..len_sym].to_vec());
    let iv = encode(&iv[..len_iv].to_vec());
    let text = sym_dec(sym.clone(), iv.clone(), bytes.value)?;
    Ok(Tomb {
        aes_key: sym,
        aes_iv: iv,
        value: encode(&text),
    })
}

pub fn private_decrypt(key: &openssl::rsa::Rsa<openssl::pkey::Private>, bytes: Tomb) -> Result<Tomb, Error> {
    let mut sym = vec![0; key.size() as usize];
    let mut iv = vec![0; key.size() as usize];
    let len_sym = key.private_decrypt(
        &decode(&bytes.aes_key)?,
        &mut sym,
        openssl::rsa::Padding::PKCS1,
    )?;
    let len_iv = key.private_decrypt(
        &decode(&bytes.aes_iv)?,
        &mut iv,
        openssl::rsa::Padding::PKCS1,
    )?;
    let sym = encode(&sym[..len_sym].to_vec());
    let iv = encode(&iv[..len_iv].to_vec());
    let text = sym_dec(sym.clone(), iv.clone(), bytes.value)?;
    Ok(Tomb {
        aes_key: sym,
        aes_iv: iv,
        value: encode(&text),
    })
}
pub fn private_encrypt(key: &Rsa<openssl::pkey::Private>, bytes: &[u8]) -> Result<Tomb, Error> {
    let mut sym_crypt = vec![0; key.size() as usize];
    let mut iv_crypt = vec![0; key.size() as usize];
    let (sym_key, iv, text) = sym_enc(String::from_utf8(bytes.to_vec())?)?;
    key.private_encrypt(
        &decode(&sym_key)?,
        &mut sym_crypt,
        openssl::rsa::Padding::PKCS1,
    )?;
    key.private_encrypt(&decode(&iv)?, &mut iv_crypt, openssl::rsa::Padding::PKCS1)?;
    Ok(Tomb {
        aes_key: encode(&sym_crypt),
        aes_iv: encode(&iv_crypt),
        value: text,
    })
}

pub fn get_public_from_file(path: &str) -> Result<Rsa<openssl::pkey::Public>, Error> {
    let mut buf = Vec::<u8>::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(Rsa::public_key_from_pem(buf.as_slice())?)
}
pub fn get_private_from_file(path: &str) -> Result<Rsa<openssl::pkey::Private>, Error> {
    let mut buf = Vec::<u8>::new();
    File::open(path)?.read_to_end(&mut buf)?;
    Ok(Rsa::private_key_from_pem(buf.as_slice())?)
}
