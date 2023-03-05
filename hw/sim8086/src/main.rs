use std::env;
use std::fs;
use std::process;

const REG_TABLE: &[&str] = &["al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di"];
const INST_STR: &str = "mov";

fn decode(bytes: Vec<u8>) -> Vec<String> {
    let mut idx: usize = 0;
    let mut insts = Vec::new();

    while idx < bytes.len() {
        let mut cur: u8 = bytes[idx];

        // ensure we're seeing something from the class of MOV REG/MEM instructions
        assert!((cur & 0xFC) == 0x88);
        let d: u8 = (cur >> 1) & 0x1;
        let w: u8 = cur & 0x1;

        idx += 1;

        cur = bytes[idx];

        let mode: u8 = (cur >> 6) & 0x3;
        // ensure we're operating in register-to-register mode
        assert!(mode == 0x3);

        let reg: u8 = (cur >> 3) & 0x7;
        let rm: u8 = cur & 0x7;
        // per docs, if d bit is 1, reg register is dest, if d bit is 0, rm register is dest.
        let reg_names: &[&str] = &[REG_TABLE[(2 * rm + w) as usize], REG_TABLE[(2 * reg + w) as usize]];
    
        insts.push(format!("{} {}, {}", INST_STR, reg_names[(d % 2) as usize], reg_names[((d + 1) % 2) as usize]));

        idx += 1;
    }

    return insts;
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_reg_mov() {
        let asm: Vec<&str> = vec![
            "mov cx, bx"
        ];
        let bytes: Vec<u8> = vec![
            0x89, 0xd9
        ];
        assert_eq!(asm, decode(bytes));
    }

    #[test]
    fn many_reg_mov() {
        let asm: Vec<&str> = vec![
            "mov cx, bx",
            "mov ch, ah",
            "mov dx, bx",
            "mov si, bx",
            "mov bx, di",
            "mov al, cl",
            "mov ch, ch",
            "mov bx, ax",
            "mov bx, si",
            "mov sp, di",
            "mov bp, ax"
        ];
        let bytes: Vec<u8> = vec![
            0x89, 0xd9, 0x88, 0xe5, 0x89, 0xda, 0x89, 0xde, 
            0x89, 0xfb, 0x88, 0xc8, 0x88, 0xed, 0x89, 0xc3, 
            0x89, 0xf3, 0x89, 0xfc, 0x89, 0xc5
        ];
        assert_eq!(asm, decode(bytes));
    }
}