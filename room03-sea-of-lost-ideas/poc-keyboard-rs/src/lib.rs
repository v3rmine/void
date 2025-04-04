#![allow(deprecated)]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;

mod keyboard;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
    }
}