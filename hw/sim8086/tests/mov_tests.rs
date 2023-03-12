
#[cfg(test)]
mod mov_tests {
    use sim8086::Buf;
    use sim8086::decode;

    #[test]
    fn mov_single_reg() {
        let asm: Vec<String> = vec![
            "mov cx, bx".to_string(),
        ];
        let buf = Buf::new(vec![
            0x89, 0xd9
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_many_reg() {
        let asm: Vec<String> = vec![
            "mov cx, bx".to_string(),
            "mov ch, ah".to_string(),
            "mov dx, bx".to_string(),
            "mov si, bx".to_string(),
            "mov bx, di".to_string(),
            "mov al, cl".to_string(),
            "mov ch, ch".to_string(),
            "mov bx, ax".to_string(),
            "mov bx, si".to_string(),
            "mov sp, di".to_string(),
            "mov bp, ax".to_string(),
        ];
        let buf = Buf::new(vec![
            0x89, 0xd9, 0x88, 0xe5, 0x89, 0xda, 0x89, 0xde, 
            0x89, 0xfb, 0x88, 0xc8, 0x88, 0xed, 0x89, 0xc3, 
            0x89, 0xf3, 0x89, 0xfc, 0x89, 0xc5
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_byte_imm_to_reg() {
        let asm: Vec<String> = vec![
            "mov cx, 12".to_string(),
            format!("mov cx, {}", (-12i16) as u16),
            "mov bl, 12".to_string(),
            format!("mov bh, {}", (-12i8) as u8),
        ];
        let buf = Buf::new(vec![
            0xb9, 0xc, 0x0, 0xb9, 0xf4, 0xff, 0xb3, 0xc, 0xb7, 0xf4
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_word_imm_to_reg() {
        let asm: Vec<String> = vec![
            "mov dx, 3948".to_string(),
            format!("mov dx, {}", (-3948i16) as u16),
        ];
        let buf = Buf::new(vec![
            0xba, 0x6c, 0xf, 0xba, 0x94, 0xf0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_src_addr_calc() {
        let asm: Vec<String> = vec![
            "mov al, [bx + si]".to_string(),
            "mov bx, [bp + di]".to_string(),
            "mov dx, [bp]".to_string(),
        ];
        let buf = Buf::new(vec![
            0x8a, 0x0, 0x8b, 0x1b, 0x8b, 0x56, 0x0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_mem_byte_disp() {
        let asm: Vec<String> = vec![
            "mov ah, [bx + si + 4]".to_string(),
        ];
        let buf = Buf::new(vec![
            0x8a, 0x60, 0x4
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_mem_word_disp() {
        let asm: Vec<String> = vec![
            "mov al, [bx + si + 4999]".to_string(),
        ];
        let buf = Buf::new(vec![
            0x8a, 0x80, 0x87, 0x13
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_dest_addr_calc() {
        let asm: Vec<String> = vec![
            "mov [bx + di], cx".to_string(),
            "mov [bp + si], cl".to_string(),
            "mov [bp], ch".to_string(),
        ];
        let buf = Buf::new(vec![
            0x89, 0x9, 0x88, 0xa, 0x88, 0x6e, 0x0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_signed_displacements() {
        let asm: Vec<String> = vec![
            "mov ax, [bx + di - 37]".to_string(),
            "mov [si - 300], cx".to_string(),
            "mov dx, [bx - 32]".to_string(),
        ];
        let buf = Buf::new(vec![
            0x8b, 0x41, 0xdb, 0x89, 0x8c, 0xd4, 0xfe, 0x8b, 0x57, 0xe0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_explicit_sizes() {
        let asm: Vec<String> = vec![
            "mov [bp + di], byte 7".to_string(),
            "mov [di + 901], word 347".to_string(),
        ];
        let buf = Buf::new(vec![
            0xc6, 0x3, 0x7, 0xc7, 0x85, 0x85, 0x3, 0x5b, 0x1
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_direct_addr() {
        let asm: Vec<String> = vec![
            "mov bp, [5]".to_string(),
            "mov bx, [3458]".to_string(),
        ];
        let buf = Buf::new(vec![
            0x8b, 0x2e, 0x5, 0x0, 0x8b, 0x1e, 0x82, 0xd
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_mem_to_acc() {
        let asm: Vec<String> = vec![
            "mov ax, [2555]".to_string(),
            "mov ax, [16]".to_string(),
        ];
        let buf = Buf::new(vec![
            0xa1, 0xfb, 0x9, 0xa1, 0x10, 0x0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn mov_acc_to_mem() {
        let asm: Vec<String> = vec![
            "mov [2554], ax".to_string(),
            "mov [15], ax".to_string(),
        ];
        let buf = Buf::new(vec![
            0xa3, 0xfa, 0x9, 0xa3, 0xf, 0x0
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }
}