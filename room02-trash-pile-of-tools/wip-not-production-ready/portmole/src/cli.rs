use clap::Parser;
use std::num::ParseIntError;

const MIN_PORT: u32 = 1;
const MAX_PORT: u32 = 65535;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub enum Commands {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(clap::Args, Debug)]
#[clap(author, version, about = "Server to expose ports")]
pub struct ServerArgs {
    #[clap(
        short,
        long,
        value_name = "portMin-portMax",
        parse(try_from_str = parse_port_range)
    )]
    pub port_range: (u32, u32),
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
}

#[derive(clap::Args, Debug)]
#[clap(author, version, about = "Client to test remote ports")]
pub struct ClientArgs {
    #[clap(
        short,
        long,
        value_name = "portMin-portMax",
        parse(try_from_str = parse_port_range)
    )]
    pub port_range: (u32, u32),
    #[clap(short = 's', long)]
    pub server: String,
    #[clap(short, long, default_value = "500")]
    pub timeout: u64,
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: usize,
}

fn parse_port_range(i: &str) -> Result<(u32, u32), &'static str> {
    if !i.contains('-') {
        panic!("Please use <-> as a separator");
    }

    match i
        .split('-')
        .map(|nb| nb.parse::<u32>())
        .collect::<Result<Vec<u32>, ParseIntError>>()
    {
        Ok(port_range) => {
            if let [port_min, port_max] = port_range[..] {
                if port_min > port_max {
                    Err("In <portMin-portMax> portMin must be smaller than portMax")
                } else if port_min < MIN_PORT {
                    Err("In <portMin-portMax> port must be not less than 1")
                } else if port_max > MAX_PORT {
                    Err("In <portMin-portMax> port must be not more than 65535")
                } else {
                    Ok((port_min, port_max))
                }
            } else {
                Err("Please provide port range in the shape <portMin-portMax>")
            }
        }
        Err(_) => Err("Please provide valid numbers in port range"),
    }
}
