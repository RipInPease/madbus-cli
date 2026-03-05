use clap::{Parser, Subcommand};
use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};


fn main() {
    let mut cmd = Args::parse();

    cmd.ip += ":502";
    let mut stream = TcpStream::connect(&cmd.ip).unwrap();
}


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    ip: String,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Command {
    ReadCoil {
        start: u16,
        count: u16
    },

    ReadDI {
        start: u16,
        count: u16
    },

    ReadHolding {
        start: u16,
        count: u16
    },

    ReadInput {
        start: u16,
        count: u16
    },

    WriteCoil {
        start: u16,
        coils: Vec<bool>
    },

    WriteHolding {
        start: u16,
        regs: Vec<u16>
    }
}