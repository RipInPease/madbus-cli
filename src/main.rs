use madbus::*;
use clap::{Parser, Subcommand};
use std::net::TcpStream;


fn main() {
    // Get the commands from the terminal
    let mut input = Args::parse();

    // Open a TCP connection with the server (slave)
    input.ip += ":";
    input.ip += &input.port.to_string();
    let mut stream = TcpStream::connect(&input.ip).unwrap();

    let cmd = get_cmd(input.command);
    Client::send_request(&mut stream, cmd, input.unit_id).unwrap();

    let response = Client::read_response(&mut stream);
    println!("{:#?}", response);
}


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t = 502)]
    port: u16,

    #[command(subcommand)]
    command: InputCommand,

    #[arg(short, long, default_value_t = 1)]
    unit_id: u8,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum InputCommand {
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

        #[arg(required = true)]
        coils: Vec<bool>
    },

    WriteHolding {
        start: u16,

        #[arg(required = true)]
        regs: Vec<u16>
    }
}


fn get_cmd(cmd: InputCommand) -> Command {
    use InputCommand as CMD;
    match cmd {
        CMD::ReadCoil{start, count}     => Command::ReadCoils   { start, count},
        CMD::ReadDI{start, count}       => Command::ReadDI      { start, count},
        CMD::ReadHolding{start, count}  => Command::ReadHolding { start, count },
        CMD::ReadInput{start, count}    => Command::ReadInput   { start, count },
        CMD::WriteCoil{start, coils}    => {
            if coils.len() > 1 {
                Command::WriteMultCoil { start, count: coils.len() as u16, vals: coils }
            } else {
                Command::WriteCoil { coil: start, state: coils[0] }
            }
        },
        CMD::WriteHolding{start, regs}  => {
            if regs.len() > 1 {
                Command::WriteMultHolding { start, count: regs.len() as u16, vals: regs }
            } else {
                Command::WriteHolding { address: start, value: regs[0] }
            }
        }
    }
}