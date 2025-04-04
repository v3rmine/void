use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Drive Headless Tools")
        .about("App to generate GSuite SA Certificates")
        .arg(
            Arg::with_name("username")
                .long("username")
                .short("u")
                .value_name("STRING")
                .required(true)
                .help("GSuite authorized user username"),
        )
        .arg(
            Arg::with_name("password")
                .long("password")
                .short("p")
                .value_name("STRING")
                .required(true)
                .help("GSuite authorized user password"),
        )
        .arg(
            Arg::with_name("project-name")
                .long("project-name")
                .short("n")
                .value_name("STRING")
                .required(true)
                .help("Name of the GSuite project"),
        )
        .arg(
            Arg::with_name("sa-id")
                .long("sa-id")
                .short("i")
                .value_name("STRING")
                .required(true)
                .help("Name of the Service Account to generate the certs"),
        )
        .arg(
            Arg::with_name("encoding")
                .long("encoding")
                .short("e")
                .value_name("ENCODING")
                .default_value("plaintext")
                .possible_values(&["plaintext", "base64"])
                .case_insensitive(true)
                .required(false)
                .help("Encoding of the response"),
        )
        .arg(
            Arg::with_name("sleeptime")
                .long("sleeptime")
                .short("s")
                .value_name("INTEGER (u64)")
                .default_value("500")
                .required(false)
                .help("Basic time to wait between actions in ms"),
        )
        .arg(
            Arg::with_name("log-level")
                .long("log-level")
                .short("v")
                .value_name("LOG_LEVEL")
                .default_value("warn")
                .possible_values(&["trace", "debug", "info", "warn", "error"])
                .case_insensitive(true)
                .required(false)
                .help("Verbosity level"),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .value_name("INTEGER (u64)")
                .default_value("3000")
                .required(false)
                .help("Timeout for actions"),
        )
        .arg(
            Arg::with_name("headless")
                .long("headless")
                .short("h")
                .takes_value(false)
                .required(false)
                .help("Running browser headless mode"),
        )
}
