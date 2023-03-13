
#[cfg(test)]
mod jmp_tests {
    use sim8086::Buf;
    use sim8086::decode;

    #[test]
    fn cond_jumps() {
        let asm: Vec<String> = vec![
            "jne 2".to_string(),
            "jne -4".to_string(),
            "jne -6".to_string(),
            "jne -4".to_string(),
            "je -2".to_string(),
            "jl -4".to_string(),
            "jle -6".to_string(),
            "jb -8".to_string(),
            "jbe -10".to_string(),
            "jp -12".to_string(),
            "jo -14".to_string(),
            "js -16".to_string(),
            "jne -18".to_string(),
            "jnl -20".to_string(),
            "jnle -22".to_string(),
            "jnb -24".to_string(),
            "jnbe -26".to_string(),
            "jnp -28".to_string(),
            "jno -30".to_string(),
            "jns -32".to_string(),
            "loopz -34".to_string(),
            "loopnz -36".to_string(),
            "jnle -38".to_string(),
            "loop -40".to_string()
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