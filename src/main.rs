use std::{net, ops};

// https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers#
const WELL_KNOWN_PORTS: ops::Range<i32> = 0..1024;
const REGISTERED_PORTS: ops::Range<i32> = 1024..49152;
const DYBAMICALLY_ALOCATED_PORTS: ops::Range<i32> = 49152..65536;
const ALL_PORTS: ops::Range<i32> = 0..65536;

fn main() {
    for port in REGISTERED_PORTS {
        let listener = net::TcpListener::bind(format!("localhost:{}", port));

        if let Err(e) = listener {
            println!("Port {}: {}", port, e);
        }
    }
}
