
#[cfg(test)]
mod alu_tests {
    use sim8086::buf::Buf;
    use sim8086::decode::decode;

    #[test]
    fn add_instr() {
        let asm: Vec<String> = vec![
            "add bx, [bx + si]".to_string(),
            "add bx, [bp]".to_string(),
            "add si, word 2".to_string(),
            "add bp, word 2".to_string(),
            "add cx, word 8".to_string(),
            "add bx, [bp]".to_string(),
            "add cx, [bx + 2]".to_string(),
            "add bh, [bp + si + 4]".to_string(),
            "add di, [bp + di + 6]".to_string(),
            "add [bx + si], bx".to_string(),
            "add [bp], bx".to_string(),
            "add [bp], bx".to_string(),
            "add [bx + 2], cx".to_string(),
            "add [bp + si + 4], bh".to_string(),
            "add [bp + di + 6], di".to_string(),
            "add [bx], byte 34".to_string(),
            "add [bp + si + 1000], word 29".to_string(),
            "add ax, [bp]".to_string(),
            "add al, [bx + si]".to_string(),
            "add ax, bx".to_string(),
            "add al, ah".to_string(),
            "add ax, 1000".to_string(),
            "add al, -30".to_string(),
            "add al, 9".to_string()
        ];
        let buf = Buf::new(vec![
            0x03, 0x18, 0x03, 0x5e, 0x00, 0x83, 0xc6, 0x02, 0x83, 0xc5, 0x02, 0x83,
            0xc1, 0x08, 0x03, 0x5e, 0x00, 0x03, 0x4f, 0x02, 0x02, 0x7a, 0x04, 0x03,
            0x7b, 0x06, 0x01, 0x18, 0x01, 0x5e, 0x00, 0x01, 0x5e, 0x00, 0x01, 0x4f,
            0x02, 0x00, 0x7a, 0x04, 0x01, 0x7b, 0x06, 0x80, 0x07, 0x22, 0x83, 0x82,
            0xe8, 0x03, 0x1d, 0x03, 0x46, 0x00, 0x02, 0x00, 0x01, 0xd8, 0x00, 0xe0,
            0x05, 0xe8, 0x03, 0x04, 0xe2, 0x04, 0x09
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn sub_instr() {
        let asm: Vec<String> = vec![
            "sub bx, [bx + si]".to_string(),
            "sub bx, [bp]".to_string(),
            "sub si, word 2".to_string(),
            "sub bp, word 2".to_string(),
            "sub cx, word 8".to_string(),
            "sub bx, [bp]".to_string(),
            "sub cx, [bx + 2]".to_string(),
            "sub bh, [bp + si + 4]".to_string(),
            "sub di, [bp + di + 6]".to_string(),
            "sub [bx + si], bx".to_string(),
            "sub [bp], bx".to_string(),
            "sub [bp], bx".to_string(),
            "sub [bx + 2], cx".to_string(),
            "sub [bp + si + 4], bh".to_string(),
            "sub [bp + di + 6], di".to_string(),
            "sub [bx], byte 34".to_string(),
            "sub [bx + di], word 29".to_string(),
            "sub ax, [bp]".to_string(),
            "sub al, [bx + si]".to_string(),
            "sub ax, bx".to_string(),
            "sub al, ah".to_string(),
            "sub ax, 1000".to_string(),
            "sub al, -30".to_string(),
            "sub al, 9".to_string()
        ];
        let buf = Buf::new(vec![
            0x2b, 0x18, 0x2b, 0x5e, 0x00, 0x83, 0xee, 0x02, 0x83, 0xed, 0x02, 0x83,
            0xe9, 0x08, 0x2b, 0x5e, 0x00, 0x2b, 0x4f, 0x02, 0x2a, 0x7a, 0x04, 0x2b,
            0x7b, 0x06, 0x29, 0x18, 0x29, 0x5e, 0x00, 0x29, 0x5e, 0x00, 0x29, 0x4f,
            0x02, 0x28, 0x7a, 0x04, 0x29, 0x7b, 0x06, 0x80, 0x2f, 0x22, 0x83, 0x29,
            0x1d, 0x2b, 0x46, 0x00, 0x2a, 0x00, 0x29, 0xd8, 0x28, 0xe0, 0x2d, 0xe8,
            0x03, 0x2c, 0xe2, 0x2c, 0x09
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

    #[test]
    fn cmp_instr() {
        let asm: Vec<String> = vec![
            "cmp bx, [bx + si]".to_string(),
            "cmp bx, [bp]".to_string(),
            "cmp si, word 2".to_string(),
            "cmp bp, word 2".to_string(),
            "cmp cx, word 8".to_string(),
            "cmp bx, [bp]".to_string(),
            "cmp cx, [bx + 2]".to_string(),
            "cmp bh, [bp + si + 4]".to_string(),
            "cmp di, [bp + di + 6]".to_string(),
            "cmp [bx + si], bx".to_string(),
            "cmp [bp], bx".to_string(),
            "cmp [bp], bx".to_string(),
            "cmp [bx + 2], cx".to_string(),
            "cmp [bp + si + 4], bh".to_string(),
            "cmp [bp + di + 6], di".to_string(),
            "cmp [bx], byte 34".to_string(),
            "cmp [4834], word 29".to_string(),
            "cmp ax, [bp]".to_string(),
            "cmp al, [bx + si]".to_string(),
            "cmp ax, bx".to_string(),
            "cmp al, ah".to_string(),
            "cmp ax, 1000".to_string(),
            "cmp al, -30".to_string(),
            "cmp al, 9".to_string()
        ];
        let buf = Buf::new(vec![
            0x3b, 0x18, 0x3b, 0x5e, 0x00, 0x83, 0xfe, 0x02, 0x83, 0xfd, 0x02, 0x83,
            0xf9, 0x08, 0x3b, 0x5e, 0x00, 0x3b, 0x4f, 0x02, 0x3a, 0x7a, 0x04, 0x3b,
            0x7b, 0x06, 0x39, 0x18, 0x39, 0x5e, 0x00, 0x39, 0x5e, 0x00, 0x39, 0x4f,
            0x02, 0x38, 0x7a, 0x04, 0x39, 0x7b, 0x06, 0x80, 0x3f, 0x22, 0x83, 0x3e,
            0xe2, 0x12, 0x1d, 0x3b, 0x46, 0x00, 0x3a, 0x00, 0x39, 0xd8, 0x38, 0xe0,
            0x3d, 0xe8, 0x03, 0x3c, 0xe2, 0x3c, 0x09
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

}