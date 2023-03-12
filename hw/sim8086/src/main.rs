use std::env;
use std::fs;
use std::process;

use sim8086::Buf;
use sim8086::decode;

#[derive(Debug)]
pub enum SimError {
    IoError(std::io::Error),
    DecodeError(sim8086::DecodeError),
}

impl From<std::io::Error> for SimError {
    fn from(err: std::io::Error) -> Self {
        SimError::IoError(err)
    }
}

impl From<sim8086::DecodeError> for SimError {
    fn from(err: sim8086::DecodeError) -> Self {
        SimError::DecodeError(err)
    }
}

fn main() -> Result<(), SimError> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Expected arg: input file path");
        process::exit(1);
    }

    let bytes: Vec<u8> = fs::read(&args[1])?;
    println!("{:#04x?}", bytes);
    let buf = Buf::new(bytes);
    println!("; {} disassembly", args[1]);
    println!("bits 16");
    for instr in decode(buf)? {
        println!("{}", instr);
    }

    Ok(())
}
