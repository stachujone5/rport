use clap::Parser;
use nix::{
    sys::signal::{kill, Signal},
    unistd::Pid,
};
use procfs::{net, process::FDTarget};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    list: bool,
    #[arg(short, long, num_args = 1..)]
    kill: Vec<u16>,
}

struct ProcessInfo {
    port: u16,
    inode: u64,
}

fn main() {
    let args = Cli::parse();
    let processes_info = get_processes_info();

    if args.list {
        for process_info in &processes_info {
            println!("{}", process_info.port);
        }
    }

    for port in args.kill {
        match processes_info.iter().find(|&item| item.port == port) {
            Some(process_info) => {
                kill_process_by_inode(process_info.inode);
            }
            None => {
                println!("Port {} is not in use!", port);
            }
        }
    }
}

fn get_processes_info() -> Vec<ProcessInfo> {
    let tcp: Vec<net::TcpNetEntry> = net::tcp().expect("Unable to get tcp entries!");
    let tcp6: Vec<net::TcpNetEntry> = net::tcp6().expect("Unable to get tcp6 entries!");
    let mut processes: Vec<ProcessInfo> = Vec::new();

    let tcp_entries: Vec<net::TcpNetEntry> = [&tcp[..], &tcp6[..]].concat();

    for entry in tcp_entries {
        if entry.state == net::TcpState::Listen {
            // Get only ports that are listening and not established
            let address = entry.local_address.to_string();

            if let Some(port_str) = address.rsplitn(2, ':').next() {
                if let Ok(port) = port_str.parse::<u16>() {
                    let process = ProcessInfo {
                        port,
                        inode: entry.inode,
                    };
                    processes.push(process);
                }
            }
        }
    }
    processes
}

fn kill_process_by_inode(target_inode: u64) {
    let processes = procfs::process::all_processes().expect("Unable to get processes!");

    for process in processes {
        if let Ok(process) = process {
            if let Ok(fds) = process.fd() {
                for fd in fds {
                    if let Ok(fd) = fd {
                        if let FDTarget::Socket(inode) = fd.target {
                            if target_inode == inode {
                                kill(Pid::from_raw(process.pid), Signal::SIGKILL).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
