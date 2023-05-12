use clap::{Parser, ValueEnum};
use std::{io, net, ops};

// https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers#
const WELL_KNOWN_PORTS: ops::Range<i32> = 0..1024;
const REGISTERED_PORTS: ops::Range<i32> = 1024..49152;
const DYBAMICALLY_ALOCATED_PORTS: ops::Range<i32> = 49152..65536;
const ALL_PORTS: ops::Range<i32> = 0..65536;

#[derive(ValueEnum, Clone)]
#[clap(rename_all = "lowercase")]
enum Flag {
    All,
    Registered,
    Dynamic,
    Wellknown,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    range: Option<Flag>,
}

fn main() {
    let args = Cli::parse();

    // When range is not specified - print all ports
    match args.range {
        Some(range) => print_used_ports(range),
        None => print_used_ports(Flag::All),
    }
}

fn print_used_ports(range: Flag) {
    let range = match range {
        Flag::All => ALL_PORTS,
        Flag::Registered => REGISTERED_PORTS,
        Flag::Dynamic => DYBAMICALLY_ALOCATED_PORTS,
        Flag::Wellknown => WELL_KNOWN_PORTS,
    };

    for port in range {
        if let Err(e) = is_port_in_use(port) {
            println!("Port {} {}", port, e);
        }
    }
}

fn is_port_in_use(port: i32) -> Result<(), io::Error> {
    let listener: Result<net::TcpListener, std::io::Error> =
        net::TcpListener::bind(format!("localhost:{}", port));

    if let Err(e) = listener {
        return Err(e);
    }

    Ok(())
}

// fn kill_process_by_port(port: i32) {
//     if let Ok(_) = is_port_in_use(port) {
//         println!("Port {} is not in use!", port);
//         return;
//     }
// }
