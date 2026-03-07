use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub ip: String,

    #[arg(short, long, default_value_t = 502)]
    pub port: u16,

    #[command(subcommand)]
    pub command: InputCommand,

    #[arg(short, long, default_value_t = 1)]
    pub unit_id: u8,
}

#[derive(Debug)]
#[derive(Subcommand)]
pub enum InputCommand {
    ReadCoil {
        start: u16,
        #[arg(default_value_t = 1)]
        count: u16
    },

    ReadDI {
        start: u16,
        #[arg(default_value_t = 1)]
        count: u16
    },

    ReadHolding {
        start: u16,
        #[arg(default_value_t = 1)]
        count: u16
    },

    ReadInput {
        start: u16,
        #[arg(default_value_t = 1)]
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