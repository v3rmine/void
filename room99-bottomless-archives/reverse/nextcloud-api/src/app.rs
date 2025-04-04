use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let username: Arg<'static, 'static> = Arg::with_name("username")
        .long("username")
        .short("u")
        .value_name("STRING")
        .required(true)
        .help("GSuite authorized user username");

    let password: Arg<'static, 'static> = Arg::with_name("password")
        .long("password")
        .short("p")
        .value_name("STRING")
        .required(true)
        .help("GSuite authorized user password");

    let response_type: Arg<'static, 'static> = Arg::with_name("response-type")
        .long("encoding")
        .short("e")
        .value_name("ENCODING")
        .default_value("plaintext")
        .possible_values(&["plaintext", "base64"])
        .case_insensitive(true)
        .required(false)
        .help("Encoding of the response");

    let sleeptime: Arg<'static, 'static> = Arg::with_name("sleeptime")
        .long("sleeptime")
        .short("s")
        .value_name("INTEGER (u64)")
        .default_value("500")
        .required(false)
        .help("Basic time to wait between actions in ms");

    let loglevel: Arg<'static, 'static> = Arg::with_name("log-level")
        .long("log-level")
        .short("v")
        .value_name("LOG_LEVEL")
        .default_value("warn")
        .possible_values(&["trace", "debug", "info", "warn", "error"])
        .case_insensitive(true)
        .required(false)
        .help("Verbosity level");

    let timeout: Arg<'static, 'static> = Arg::with_name("timeout")
        .long("timeout")
        .short("t")
        .value_name("INTEGER (u64)")
        .default_value("3000")
        .required(false)
        .help("Timeout for actions");

    App::new("Drive Headless Tools")
        .about("App to generate GSuite SA Certificates")
        .subcommand(SubCommand::with_name("create").args(&[
            username.clone(),
            password.clone(),
            sleeptime.clone(),
            loglevel.clone(),
            timeout.clone(),
        ]))
        .subcommand(SubCommand::with_name("cookies").args(&[
            username,
            password,
            response_type,
            sleeptime,
            loglevel,
            timeout,
        ]))
}
