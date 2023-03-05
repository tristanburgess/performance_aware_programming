use std::env;
use std::fs;
use std::process;

use sim8086::decode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Expected arg: input file path");
        process::exit(1);
    }

    let bytes: Vec<u8> = fs::read(&args[1]).expect(&format!("failed to open input file {}", args[1]));

    println!("; {} disassembly", args[1]);
    println!("bits 16");
    for instr in decode(bytes) {
        println!("{}", instr);
    }
}
