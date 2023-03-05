const REG_TABLE: &[&str] = &["al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di"];
const EA_CALC_TABLE: &[[&str; 8]; 3] = &[
    ["al", "ax", "cl", "cx", "dl", "dx", "bl", "bx"],
    ["al", "ax", "cl", "cx", "dl", "dx", "bl", "bx"],
    ["al", "ax", "cl", "cx", "dl", "dx", "bl", "bx"],
];
const INST_STR: &str = "mov";

pub fn decode(bytes: Vec<u8>) -> Vec<String> {
    let mut idx: usize = 0;
    let mut insts = Vec::new();

    while idx < bytes.len() {
        if (bytes[idx] & 0xFC) == 0x88 {
            let d: u8 = (bytes[idx] >> 1) & 0x1;
            let w: u8 = bytes[idx] & 0x1;
    
            idx += 1;
    
            let mode: u8 = (bytes[idx] >> 6) & 0x3;
    
            let reg: u8 = (bytes[idx] >> 3) & 0x7;
            let rm: u8 = bytes[idx] & 0x7;
            // per docs, if d bit is 1, reg register is dest, if d bit is 0, rm register is dest.
            let reg_names: &[&str] = &[REG_TABLE[(2 * rm + w) as usize], REG_TABLE[(2 * reg + w) as usize]];
        
            insts.push(format!("{} {}, {}", INST_STR, reg_names[(d % 2) as usize], reg_names[((d + 1) % 2) as usize]));
    
            idx += 1;
        } else if (bytes[idx] & 0xF0) == 0xB0 {
            let w: u8 = (bytes[idx] >> 3) & 0x1;
            let reg: u8 = bytes[idx] & 0x7;
    
            let reg_name: &str = REG_TABLE[(2 * reg + w) as usize];

            idx += 1;

            let mut data: u16 = bytes[idx].into();
            if w == 1 {
                idx += 1;

                data = data | ((bytes[idx] as u16) << 8);
            }

            insts.push(format!("{} {}, {}", INST_STR, reg_name, data));

            idx += 1;
        } else {
            // TODO: proper error handling
            panic!("Unsupported instruction found! Exiting...");
        }
    }

    return insts;
}