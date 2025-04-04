extern crate clap;

use clap::Shell;

include!("src/app.rs");

fn main() {
    let mut app = build_cli();
    app.gen_completions("gsuite-cert", Shell::PowerShell, "./");
}