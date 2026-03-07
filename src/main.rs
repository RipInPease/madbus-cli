use madbus::*;
use clap::Parser;
use std::net::TcpStream;

mod input;
use input::*;


fn main() {
    // Get the commands from the terminal
    let mut input = Args::parse();

    // Open a TCP connection with the server (slave)
    input.ip += ":";
    input.ip += &input.port.to_string();
    let mut stream = TcpStream::connect(&input.ip).unwrap();

    let cmd = get_cmd(input.command);
    Client::send_request(&mut stream, cmd.clone(), input.unit_id).unwrap();

    let response = Client::read_response(&mut stream);
    print_res(response, cmd);
}


// Turns the input command to a madbus command
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


fn print_res(res: Result<(Response, u8, u16), Exception>, cmd: Command) {
    if let Err(err) = res {
        print_err(err);
        return;
    }

    let res = res.unwrap();
    let response = res.0;
    let uid = res.1;

    match response {
        // Prints the response for ReadCoils
        Response::ReadCoils { status } => {
            if !matches!(cmd, Command::ReadCoils{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            let (_, count) = cmd.unwrap_read_coil();
            println!("Coil states from unit {}:", uid);
            for i in 0..count {
                println!("\t{}: {}", 10000+i, status[i as usize])
            }
        },

        // Prints the response for ReadDI
        Response::ReadDI { status } => {
            if !matches!(cmd, Command::ReadDI{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            let (_, count) = cmd.unwrap_read_di();
            println!("DI states from unit {}:", uid);
            for i in 0..count {
                println!("\t{}: {}", 20000+i, status[i as usize])
            }
        },

        // Prints the response for ReadHolding
        Response::ReadHolding { status } => {
            if !matches!(cmd, Command::ReadHolding{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            let (_, count) = cmd.unwrap_read_holding();
            println!("Holding reg values from unit {}:", uid);
            for i in 0..count {
                println!("\t{}: {}", 40000+i, status[i as usize])
            }
        },

        // Prints the response for ReadInput
        Response::ReadInput{ status } => {
            if !matches!(cmd, Command::ReadInput{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            let (_, count) = cmd.unwrap_read_input();
            println!("Input reg values from unit {}:", uid);
            for i in 0..count {
                println!("\t{}: {}", 30000+i, status[i as usize])
            }
        },

        // Prints the response for WriteCoil
        Response::WriteCoil{ coil, state } => {
            if !matches!(cmd, Command::WriteCoil{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            println!("Unit {} wrote the coil {} to {}", uid, 10000+coil, state);
        },

        // Prints the response for WriteHolding
        Response::WriteHolding{ address, value } => {
            if !matches!(cmd, Command::WriteHolding{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            println!("Unit {} wrote the register {} to {}", uid, 40000+address, value);
        },

        // Prints the response for WriteMultCoil
        Response::WriteMultCoil{ start, count } => {
            if !matches!(cmd, Command::WriteMultCoil{..}) {
                println!("The unit returned the wrong function code, that silly lil guy");
                return;
            }

            println!("Unit {} wrote the registers {}..{}", uid, 40000+start, 40000+count-1);
        },

        _ => ()
    }

}

fn print_err(err: Exception) {
    match err {
        Exception::IllegalCode => { 
            println!("Received exception code from unit: IllegalCode");
        },
        Exception::IllegalAddress => { 
            println!("Received exception code from unit: IllegalAddress");
        },
        Exception::IllegalDataValue => { 
            println!("Received exception code from unit: IllegalDataValue");
        },
        Exception::ServerFailure => { 
            println!("Received exception code from unit: ServerFailure");
        },
        Exception::Acknowledge => { 
            println!("Received exception code from unit: Acknowledge");
        },
        Exception::ServerBusy => { 
            println!("Received exception code from unit: ServerBusy");
        },
        Exception::GatewayUnavail => { 
            println!("Received exception code from unit: GatewayUnavail");
        },
        Exception::BadDevice => { 
            println!("Received exception code from unit: BadDevice");
        },
        Exception::FailedRead => { 
            println!("Failed to read response due to either insufficiant number of bytes received or bytes not in accordance to the modbus standard received");
        },
        Exception::IOError(_) => {
            println!("Failed to read due to an unexpected IOError with the TCPStream")
        }
    }
}