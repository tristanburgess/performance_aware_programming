
#[cfg(test)]
mod jmp_tests {
    use sim8086::Buf;
    use sim8086::decode;

    #[test]
    fn uncond_jumps() {
        let asm: Vec<String> = vec![
            "test_label0:".to_string(),
            "jnz test_label1".to_string(),
            "jnz test_label0".to_string(),
            "test_label1:".to_string(),
            "jnz test_label0".to_string(),
            "jnz test_label1".to_string(),
            "label:".to_string(),
            "je label".to_string(),
            "jl label".to_string(),
            "jle label".to_string(),
            "jb label".to_string(),
            "jbe label".to_string(),
            "jp label".to_string(),
            "jo label".to_string(),
            "js label".to_string(),
            "jne label".to_string(),
            "jnl label".to_string(),
            "jg label".to_string(),
            "jnb label".to_string(),
            "ja label".to_string(),
            "jnp label".to_string(),
            "jno label".to_string(),
            "jns label".to_string(),
            "loop label".to_string(),
            "loopz label".to_string(),
            "loopnz label".to_string(),
            "jcxz label".to_string()
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