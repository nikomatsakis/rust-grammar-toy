use super::join;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Ty {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use super::super::join;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_26_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_3e_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22dyn_22(&'input str),
        Term_22fn_22(&'input str),
        Term_22impl_22(&'input str),
        Termr_23_22_5c_27_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(&'input str),
        Nt_22_2c_22_3f(::std::option::Option<&'input str>),
        Nt_28_29(()),
        NtBound(String),
        NtBounds(String),
        NtComma1_3cTy_3e(Vec<String>),
        NtComma_3cTy_3e(Vec<String>),
        NtId(String),
        NtLifetime(String),
        NtLifetimes(Vec<String>),
        NtTy(String),
        NtTy0(String),
        NtTy1(String),
        NtTyArgs(Vec<String>),
        Nt____Ty(String),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 1
        0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, -20, 16, -20, 0, 0, -20, 0, 0, 0, 0, 0,
        // State 3
        0, 0, -28, -4, -28, 0, 17, -28, 0, 0, 0, 0, 0,
        // State 4
        -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6,
        // State 5
        -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 6
        -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 7
        8, 20, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 14,
        // State 8
        8, 9, 0, 0, 0, 0, 0, 0, 23, 11, 24, 13, 14,
        // State 9
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 10
        0, 28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 12
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 13
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 14
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 15
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 16
        8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 17
        0, 0, -28, 0, -28, 0, 37, -28, 0, 0, 0, 0, 0,
        // State 18
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
        // State 19
        8, 9, 0, 0, 0, 0, 0, 0, 23, 11, 24, 13, 14,
        // State 20
        0, 0, 39, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 23
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 24
        0, 0, -22, 16, -22, 0, 0, -22, 0, 0, 0, 0, 0,
        // State 25
        0, 0, -4, -4, -4, 0, 43, -4, 0, 0, 0, 0, 0,
        // State 26
        0, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 14,
        // State 27
        8, 9, -13, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 28
        0, 0, -21, 16, -21, 0, 0, -21, 0, 0, 0, 0, 0,
        // State 29
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 30
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 31
        0, 0, 0, 0, 47, 0, 0, -38, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, -6, -17, 0, 0, -17, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 48, 0, 0, -37, 0, 0, 0, 0, 0,
        // State 34
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0,
        // State 36
        8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 37
        0, 0, 51, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, -31, -7, -31, 0, 0, -31, 0, 0, 0, 0, 0,
        // State 39
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 40
        0, 0, 52, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 53, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        8, 9, 0, 0, 0, 0, 0, 0, 10, 11, 12, 13, 14,
        // State 43
        0, 0, 55, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, -14, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        8, 9, -12, 0, -12, 0, 0, -12, 10, 11, 12, 13, 14,
        // State 47
        8, 9, 0, 0, 0, 0, 0, -36, 10, 11, 12, 13, 14,
        // State 48
        0, 0, -29, -5, -29, 0, 0, -29, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 60, 0, 0, 0, 0, 0,
        // State 50
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 51
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 52
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0,
        // State 54
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 55
        0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 57
        0, 0, 0, 0, 47, 0, 0, -35, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, -6, -18, 0, 0, -18, 0, 0, 0, 0, 0,
        // State 59
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 60
        -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5,
        // State 61
        8, 20, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 14,
        // State 62
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        0,
        -20,
        -28,
        -6,
        -39,
        -19,
        0,
        0,
        0,
        0,
        0,
        -16,
        -15,
        0,
        0,
        0,
        -28,
        -27,
        0,
        0,
        0,
        0,
        0,
        -22,
        -4,
        0,
        0,
        -21,
        -8,
        -9,
        0,
        0,
        0,
        -10,
        0,
        0,
        0,
        -31,
        -30,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -29,
        0,
        -31,
        -33,
        -32,
        0,
        -7,
        0,
        -11,
        0,
        0,
        -29,
        -5,
        0,
        -34,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 3, 0, 0, 4, 5, 0, 6, 0, 7, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 19, 0, 0,
        // State 8
        0, 0, 2, 21, 0, 0, 4, 5, 0, 0, 0, 22, 0, 0,
        // State 9
        0, 0, 2, 25, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 2, 29, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 30, 0, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 31, 0, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 2, 3, 32, 0, 4, 33, 34, 35, 0, 7, 36, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 2, 38, 0, 0, 4, 5, 0, 0, 0, 22, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 2, 41, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 2, 42, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 2, 44, 0, 0, 26, 5, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 2, 3, 45, 46, 4, 5, 0, 35, 0, 7, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 2, 3, 32, 0, 4, 33, 34, 35, 0, 7, 50, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 2, 3, 32, 0, 4, 33, 34, 35, 0, 7, 54, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 2, 3, 0, 0, 4, 5, 0, 57, 0, 7, 0, 0,
        // State 47
        0, 0, 2, 3, 58, 0, 4, 59, 0, 35, 0, 7, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 63, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""+""###,
            r###"",""###,
            r###""->""###,
            r###""<""###,
            r###"">""###,
            r###""dyn""###,
            r###""fn""###,
            r###""impl""###,
            r###"r#"\'[a-zA-Z][a-zA-Z0-9]*"#"###,
            r###"r#"[a-zA-Z][a-zA-Z0-9]*"#"###,
        ];
        __ACTION[(__state * 13)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Ty<
        'input,
    >(
        input: &'input str,
    ) -> Result<String, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (2, _) if true => 0,
                (3, _) if true => 1,
                (4, _) if true => 2,
                (5, _) if true => 3,
                (6, _) if true => 4,
                (7, _) if true => 5,
                (8, _) if true => 6,
                (9, _) if true => 7,
                (10, _) if true => 8,
                (11, _) if true => 9,
                (12, _) if true => 10,
                (0, _) if true => 11,
                (1, _) if true => 12,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 13 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2d_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22dyn_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22fn_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22impl_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<String,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ","? = "," => ActionFn(24);
                let __sym0 = __pop_Term_22_2c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_22_2c_22_3f(__nt), __end));
                0
            }
            2 => {
                // ","? =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_22_2c_22_3f(__nt), __end));
                0
            }
            3 => {
                // () =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_29(__nt), __end));
                1
            }
            4 => {
                // Bound = Id => ActionFn(8);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBound(__nt), __end));
                2
            }
            5 => {
                // Bound = Id, "<", TyArgs, ">" => ActionFn(9);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtTyArgs(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtBound(__nt), __end));
                2
            }
            6 => {
                // Bound = Lifetime => ActionFn(10);
                let __sym0 = __pop_NtLifetime(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBound(__nt), __end));
                2
            }
            7 => {
                // Bound = "(", Bounds, ")" => ActionFn(11);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBound(__nt), __end));
                2
            }
            8 => {
                // Bounds = Bound, "+", Bound => ActionFn(6);
                let __sym2 = __pop_NtBound(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtBound(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBounds(__nt), __end));
                3
            }
            9 => {
                // Bounds = Bounds, "+", Bound => ActionFn(7);
                let __sym2 = __pop_NtBound(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtBounds(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBounds(__nt), __end));
                3
            }
            10 => {
                // Comma1<Ty> = Ty => ActionFn(26);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                4
            }
            11 => {
                // Comma1<Ty> = Comma1<Ty>, ",", Ty => ActionFn(27);
                let __sym2 = __pop_NtTy(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                4
            }
            12 => {
                // Comma1<Ty> = Comma1<Ty>, "," => ActionFn(28);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma1_3cTy_3e(__nt), __end));
                4
            }
            13 => {
                // Comma<Ty> =  => ActionFn(34);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action34::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                5
            }
            14 => {
                // Comma<Ty> = Comma1<Ty> => ActionFn(30);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cTy_3e(__nt), __end));
                5
            }
            15 => {
                // Id = r#"[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtId(__nt), __end));
                6
            }
            16 => {
                // Lifetime = r#"\'[a-zA-Z][a-zA-Z0-9]*"# => ActionFn(23);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLifetime(__nt), __end));
                7
            }
            17 => {
                // Lifetimes = Lifetime => ActionFn(20);
                let __sym0 = __pop_NtLifetime(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLifetimes(__nt), __end));
                8
            }
            18 => {
                // Lifetimes = Lifetimes, ",", Lifetime => ActionFn(21);
                let __sym2 = __pop_NtLifetime(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtLifetimes(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLifetimes(__nt), __end));
                8
            }
            19 => {
                // Ty = Ty1 => ActionFn(35);
                let __sym0 = __pop_NtTy1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                9
            }
            20 => {
                // Ty = Bounds => ActionFn(36);
                let __sym0 = __pop_NtBounds(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                9
            }
            21 => {
                // Ty = "impl", Bounds => ActionFn(37);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action37::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                9
            }
            22 => {
                // Ty = "dyn", Bounds => ActionFn(38);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22dyn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action38::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTy(__nt), __end));
                9
            }
            23 => {
                // Ty0 = Ty1 => ActionFn(2);
                let __sym0 = __pop_NtTy1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy0(__nt), __end));
                10
            }
            24 => {
                // Ty0 = Bounds => ActionFn(3);
                let __sym0 = __pop_NtBounds(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy0(__nt), __end));
                10
            }
            25 => {
                // Ty0 = "impl", Bounds => ActionFn(4);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22impl_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTy0(__nt), __end));
                10
            }
            26 => {
                // Ty0 = "dyn", Bounds => ActionFn(5);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22dyn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTy0(__nt), __end));
                10
            }
            27 => {
                // Ty1 = "&", Ty1 => ActionFn(12);
                let __sym1 = __pop_NtTy1(__symbols);
                let __sym0 = __pop_Term_22_26_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            28 => {
                // Ty1 = Id => ActionFn(13);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            29 => {
                // Ty1 = Id, "<", TyArgs, ">" => ActionFn(14);
                let __sym3 = __pop_Term_22_3e_22(__symbols);
                let __sym2 = __pop_NtTyArgs(__symbols);
                let __sym1 = __pop_Term_22_3c_22(__symbols);
                let __sym0 = __pop_NtId(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            30 => {
                // Ty1 = "(", Ty1, ")" => ActionFn(39);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtTy1(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            31 => {
                // Ty1 = "(", Bounds, ")" => ActionFn(40);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtBounds(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action40::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            32 => {
                // Ty1 = "(", "impl", Bounds, ")" => ActionFn(41);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtBounds(__symbols);
                let __sym1 = __pop_Term_22impl_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action41::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            33 => {
                // Ty1 = "(", "dyn", Bounds, ")" => ActionFn(42);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtBounds(__symbols);
                let __sym1 = __pop_Term_22dyn_22(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action42::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            34 => {
                // Ty1 = "fn", "(", Comma<Ty>, ")", "->", Ty1 => ActionFn(16);
                let __sym5 = __pop_NtTy1(__symbols);
                let __sym4 = __pop_Term_22_2d_3e_22(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtComma_3cTy_3e(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22fn_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtTy1(__nt), __end));
                11
            }
            35 => {
                // TyArgs = Lifetimes, ",", Comma1<Ty> => ActionFn(17);
                let __sym2 = __pop_NtComma1_3cTy_3e(__symbols);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtLifetimes(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTyArgs(__nt), __end));
                12
            }
            36 => {
                // TyArgs = Lifetimes, "," => ActionFn(32);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtLifetimes(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTyArgs(__nt), __end));
                12
            }
            37 => {
                // TyArgs = Lifetimes => ActionFn(33);
                let __sym0 = __pop_NtLifetimes(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTyArgs(__nt), __end));
                12
            }
            38 => {
                // TyArgs = Comma1<Ty> => ActionFn(19);
                let __sym0 = __pop_NtComma1_3cTy_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTyArgs(__nt), __end));
                12
            }
            39 => {
                // __Ty = Ty => ActionFn(0);
                let __sym0 = __pop_NtTy(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 14 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22dyn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22dyn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22fn_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22fn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22impl_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22impl_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_22_2c_22_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<&'input str>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_22_2c_22_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (), usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBound<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBound(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBounds<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBounds(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma1_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma1_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cTy_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cTy_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtId<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtId(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLifetime<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLifetime(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLifetimes<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLifetimes(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTy1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTy1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTyArgs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTyArgs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ty<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ty(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Ty::parse_Ty;
mod __intern_token {
    #![allow(unused_imports)]
    use super::join;
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\')(?u:[A-Za-z])(?u:[0-9A-Za-z])*",
                "^(?u:[A-Za-z])(?u:[0-9A-Za-z])*",
                "^(?u:\\&)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\->)",
                "^(?u:<)",
                "^(?u:>)",
                "^(?u:dyn)",
                "^(?u:fn)",
                "^(?u:impl)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\')(?u:[A-Za-z])(?u:[0-9A-Za-z])*").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])(?u:[0-9A-Za-z])*").unwrap(),
                __regex::Regex::new("^(?u:\\&)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\->)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:dyn)").unwrap(),
                __regex::Regex::new("^(?u:fn)").unwrap(),
                __regex::Regex::new("^(?u:impl)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 13 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    format!("impl {}", __0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    format!("dyn {}", __0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
) -> String
{
    format!("({} + {})", l, r)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
) -> String
{
    format!("({} + {})", l, r)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("{}<{}>", n, join(a))
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("({})", __0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
) -> String
{
    format!("&{}", __0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> String
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, n, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("{}<{}>", n, join(a))
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    format!("({})", __0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, r, _): (usize, String, usize),
) -> String
{
    format!("fn({}) -> {}", join(a), r)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, Vec<String>, usize),
) -> Vec<String>
{
    {
        let (mut a, b) = (__0, __1);
        a.extend(b);
        a
    }
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, ::std::option::Option<&'input str>, usize),
) -> Vec<String>
{
    (__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<String>, usize),
) -> Vec<String>
{
    (__0)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, String, usize),
) -> Vec<String>
{
    {
        let (mut v, l) = (__0, __1);
        v.push(l);
        v
    }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ::std::option::Option<&'input str>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<&'input str>
{
    None
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, b, _): (usize, String, usize),
) -> Vec<String>
{
    {
        let mut v = v;
        v.push(b);
        v
    }
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, Vec<String>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<String>
{
    v
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, (), usize),
) -> Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<String>, usize),
) -> Vec<String>
{
    (__0)
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ()
{
    ()
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<String>, usize),
    __1: (usize, &'input str, usize),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action24(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    __0: (usize, Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action31(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action2(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, String, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action3(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action4(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
) -> String
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action5(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action2(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, String, usize),
    __2: (usize, &'input str, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action3(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action4(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, String, usize),
    __3: (usize, &'input str, usize),
) -> String
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action5(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}
