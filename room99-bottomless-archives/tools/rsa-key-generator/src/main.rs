extern crate openssl;

use ah_tools::passwords::hash_password;
use ah_tools::security::{
    get_private_from_file, get_public_from_file, private_decrypt, private_encrypt, public_decrypt,
    public_encrypt, Tomb,
};
use base64::{decode, encode};
use console::{style, Term};
use dialoguer;
use openssl::rsa::Rsa;
use openssl_probe::init_ssl_cert_env_vars;
use std::io::Write;
use std::mem::drop;
use std::sync::{Arc, Mutex};
use std::thread::{sleep, spawn, JoinHandle};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "n", long = "key-name")]
    name: Option<String>,

    #[structopt(short = "s", long = "key-size")]
    size: Option<u32>,
}

type Error = Box<dyn std::error::Error>;

macro_rules! c {
    ($color:ident, $text:expr) => {
        format!("{}", style($text).$color()).as_str()
    };
}

fn main() -> Result<(), Error> {
    init_ssl_cert_env_vars();
    let opt = Opt::from_args();
    let term = Arc::new(Mutex::new(Term::stdout()));

    if opt.name.is_none() && opt.size.is_none() {
        let mut threads: Vec<JoinHandle<Result<(), String>>> = Vec::new();

        let term_lock = term.lock().unwrap();
        term_lock.clear_screen()?;
        term_lock.write_line(c!(yellow, "=================================="))?;
        term_lock.write_line(c!(yellow, "     Welcome in Hume Debug!"))?;
        term_lock.write_line(c!(yellow, "=================================="))?;
        term_lock.write_line("")?;
        drop(term_lock);

        loop {
            let term_lock = term.lock().unwrap();
            let dial = dialoguer::Select::new()
                .with_prompt(c!(green, "Select a command"))
                .items(&[
                    "regen pub & private",
                    "query API",
                    "encrypt",
                    "decrypt",
                    "hash",
                    "exit",
                ])
                .interact_on_opt(&term_lock);
            drop(term_lock);
            let is_ok = dial.is_ok();
            let is_some = match dial {
                Ok(x) => x.is_some(),
                Err(_) => false,
            };

            if is_ok && !is_some {
                for thread in threads {
                    thread
                        .join()
                        .map_err(|_| "ERROR WHILE FINISHING THE THREADS")??;
                }
                let term_lock = term.lock().unwrap();
                term_lock.clear_screen()?;
                drop(term_lock);
                std::process::exit(0);
            } else if is_ok && is_some {
                match dial.unwrap().unwrap() {
                    0 => threads.push(regen(term.clone())?),
                    1 => query(term.clone())?,
                    2 => encrypt_fn(term.clone())?,
                    3 => decrypt_fn(term.clone())?,
                    4 => hash_fn(term.clone())?,
                    5 | _ => {
                        for thread in threads {
                            thread
                                .join()
                                .map_err(|_| "ERROR WHILE FINISHING THE THREADS")??;
                        }
                        std::process::exit(0);
                    }
                };
            }
        }
    } else {
        let rsa = Rsa::generate(opt.size.unwrap()).unwrap();
        let rsa_priv: Vec<u8> = rsa.clone().private_key_to_pem()?;
        let rsa_public: Vec<u8> = rsa.clone().public_key_to_pem()?;
        std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(
            format!("./{}.prv", opt.name.clone().unwrap()).as_str(),
        ))?)
        .write_all(rsa_priv.as_slice())?;
        std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(
            format!("./{}.pub", opt.name.unwrap()).as_str(),
        ))?)
        .write_all(rsa_public.as_slice())?;
    }

    Ok(())
}

fn regen(term: Arc<Mutex<Term>>) -> Result<JoinHandle<Result<(), String>>, Error> {
    let term_lock = term.lock().unwrap();
    let key_size = dialoguer::Input::new()
        .default(4096)
        .show_default(true)
        .with_prompt(c!(green, "Key size in bytes"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);
    let term_lock = term.lock().unwrap();
    let filename = dialoguer::Input::new()
        .default("rsa".to_owned())
        .show_default(true)
        .with_prompt(c!(green, "Key name"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);
    let thread = spawn(move || -> Result<(), String> {
        move || -> Result<(), Error> {
            let rsa = Rsa::generate(key_size)?;
            let rsa_priv: Vec<u8> = rsa.clone().private_key_to_pem()?;
            let rsa_public: Vec<u8> = rsa.clone().public_key_to_pem()?;
            std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(
                format!("./{}.prv", filename).as_str(),
            ))?)
            .write_all(rsa_priv.as_slice())?;
            std::io::BufWriter::new(std::fs::File::create(&std::path::Path::new(
                format!("./{}.pub", filename).as_str(),
            ))?)
            .write_all(rsa_public.as_slice())?;

            let term = term.lock().unwrap();
            term.write_line(c!(red, "Done generating the key"))?;
            sleep(std::time::Duration::from_millis(1500));
            term.clear_line()?;
            drop(term);
            Ok(())
        }()
        .map_err(|_| "ERROR GENERATING THE KEY".to_owned())?;
        Ok(())
    });

    Ok(thread)
}

fn query(term: Arc<Mutex<Term>>) -> Result<(), Error> {
    let term_lock = term.lock().unwrap();
    let msg: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your message to encrypt"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);

    // Rands
    let private = get_private_from_file("./rsa.prv")?;

    let data = private_encrypt(&private, msg.as_bytes())?;
    let data = encode(serde_json::to_string(&data)?.as_bytes());

    let term_lock = term.lock().unwrap();
    term_lock.write_line(c!(red, data.clone()))?;
    drop(term_lock);

    let public = get_public_from_file("./rsa.pub")?;

    let data = serde_json::from_str::<Tomb>(String::from_utf8(decode(&data)?)?.as_str())?;
    let data = public_decrypt(&public, data)?;
    let data = String::from_utf8(decode(&data.value)?)?;

    let term_lock = term.lock().unwrap();
    term_lock.write_line(c!(red, data))?;
    drop(term_lock);

    Ok(())
}

fn encrypt_fn(term: Arc<Mutex<Term>>) -> Result<(), Error> {
    let term_lock = term.lock().unwrap();
    let msg: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your message to encrypt"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);
    let term_lock = term.lock().unwrap();
    let path: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your public key file"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);

    let msg = if std::path::Path::new(&msg).exists() {
        std::fs::read_to_string(std::path::Path::new(&msg))?
    } else {
        msg
    };

    let key = get_public_from_file(path.as_str())?;
    let data = public_encrypt(&key, msg.as_bytes())?;
    println!("{:?}", data);
    let data = encode(&serde_json::to_string(&data)?.as_bytes());

    let term_lock = term.lock().unwrap();
    term_lock.write_line(c!(red, data))?;
    drop(term_lock);

    Ok(())
}

fn decrypt_fn(term: Arc<Mutex<Term>>) -> Result<(), Error> {
    let term_lock = term.lock().unwrap();
    let msg: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your message to decrypt in base64"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);
    let term_lock = term.lock().unwrap();
    let path: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your private key file"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);

    let msg = if std::path::Path::new(&msg).exists() {
        std::fs::read_to_string(std::path::Path::new(&msg))?
    } else {
        msg
    };

    let msg = serde_json::from_str::<Tomb>(String::from_utf8(decode(&msg)?)?.as_str())?;

    let key = get_private_from_file(path.as_str())?;
    let decrypted = private_decrypt(&key, msg)?;

    let term_lock = term.lock().unwrap();
    term_lock.write_line(c!(red, String::from_utf8(decode(&decrypted.value)?)?))?;
    drop(term_lock);

    Ok(())
}

fn hash_fn(term: Arc<Mutex<Term>>) -> Result<(), Error> {
    let term_lock = term.lock().unwrap();
    let msg: String = dialoguer::Input::new()
        .with_prompt(c!(green, "Your message to hash"))
        .allow_empty(false)
        .interact_on(&term_lock)?;
    drop(term_lock);
    let x = hash_password(msg.as_str())?;
    let term_lock = term.lock().unwrap();
    term_lock.write_line(c!(red, x.as_str()))?;
    drop(term_lock);

    Ok(())
}
