
#[cfg(test)]
mod jmp_tests {
    use sim8086::buf::Buf;
    use sim8086::decode::decode;

    #[test]
    fn cond_jumps() {
        let asm: Vec<String> = vec![
            "label_2:".to_string(),
            "jne label_1".to_string(),
            "jne label_2".to_string(),
            "label_1:".to_string(),
            "jne label_2".to_string(),
            "jne label_1".to_string(),
            "label_3:".to_string(),
            "je label_3".to_string(),
            "jl label_3".to_string(),
            "jle label_3".to_string(),
            "jb label_3".to_string(),
            "jbe label_3".to_string(),
            "jp label_3".to_string(),
            "jo label_3".to_string(),
            "js label_3".to_string(),
            "jne label_3".to_string(),
            "jnl label_3".to_string(),
            "jnle label_3".to_string(),
            "jnb label_3".to_string(),
            "jnbe label_3".to_string(),
            "jnp label_3".to_string(),
            "jno label_3".to_string(),
            "jns label_3".to_string(),
            "loop label_3".to_string(),
            "loopz label_3".to_string(),
            "loopnz label_3".to_string(),
            "jcxz label_3".to_string()
        ];
        let buf = Buf::new(vec![
            0x75, 0x02, 0x75, 0xfc, 0x75, 0xfa, 0x75, 0xfc, 0x74, 0xfe, 0x7c, 0xfc,
            0x7e, 0xfa, 0x72, 0xf8, 0x76, 0xf6, 0x7a, 0xf4, 0x70, 0xf2, 0x78, 0xf0,
            0x75, 0xee, 0x7d, 0xec, 0x7f, 0xea, 0x73, 0xe8, 0x77, 0xe6, 0x7b, 0xe4,
            0x71, 0xe2, 0x79, 0xe0, 0xe2, 0xde, 0xe1, 0xdc, 0xe0, 0xda, 0xe3, 0xd8
        ]);
        assert_eq!(asm, decode(buf).unwrap());
    }

}