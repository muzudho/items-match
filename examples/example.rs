extern crate rattle_items_match;

use rattle_items_match::{
    ActualBuilder as Actual, Condition as Cnd, ConditionsBuilder as Cnds, Control as Co,
    ExpectedBuilder as Expected, MachineBuilder as Ma, Operator as Op, RangeIncludesMax, Repeat,
    RoutineBuilder as Ro,
};

fn main() {
    println!("Start.");

    // `Actual` is sequence only.
    // 比較対象値は シーケンスのみです。
    let ac1 = Actual::default() // "    1"
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac2 = Actual::default() // "\t   1"
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac3 = Actual::default() // 'x   1'
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let ac4 = Actual::default().push(&'A').build(); // 'A'
    let ac5 = Actual::default().push(&'B').push(&'C').build(); // 'BC'
    let ac6 = Actual::default().push(&'d').push(&'e').build(); // 'de'
    let ac7 = Actual::default().push(&'f').push(&'g').push(&'h').build(); // 'fgh'
    let ac8 = Actual::default().push(&'\r').push(&'\n').build(); // "\r\n"
    let ac9 = Actual::default() // '# Comment あ.'
        .push(&'#')
        .push(&' ')
        .push(&'C')
        .push(&'o')
        .push(&'m')
        .push(&'m')
        .push(&'e')
        .push(&'n')
        .push(&'t')
        .push(&' ')
        .push(&'あ')
        .push(&'.')
        .build();
    let ac10 = Actual::default() // 'No-1_0' - Unquloted key.
        .push(&'N')
        .push(&'o')
        .push(&'-')
        .push(&'1')
        .push(&'_')
        .push(&'0')
        .build();

    // https://github.com/toml-lang/toml/blob/1.0.0-rc.3/toml.abnf
    // TODO 18 toml = expression *( newline expression )
    // TODO 20 expression =  ws [ comment ]
    // TODO 21 expression =/ ws keyval ws [ comment ]
    // TODO 22 expression =/ ws table ws [ comment ]

    // TODO 26 ws = *wschar

    // Whitespace characters.
    // 27 wschar =  %x20  ; Space
    // 28 wschar =/ %x09  ; Horizontal tab
    let wschar = Cnds::default()
        .push(&Cnd::Pin('\t'))
        .push(&Cnd::Pin(' '))
        .build();

    // Newline.
    // 32 newline =  %x0A     ; LF
    // 33 newline =/ %x0D.0A  ; CRLF
    let newline = Cnds::default()
        .push(&Cnd::Pin('\n')) // LF
        .push(&Cnd::Seq(vec!['\r', '\n'])) // CR LF
        .build();

    // 37 comment-start-symbol = %x23 ; #
    let comment_start_symbol = Cnd::Pin('#'); // #

    // 38 non-ascii = %x80-D7FF / %xE000-10FFFF
    let non_ascii = Cnds::default()
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default()
                .min(&(0x80 as char))
                .max(&'\u{D7FF}')
                .build(),
        ))
        .push(&Cnd::RangeIncludesMax(
            RangeIncludesMax::default()
                .min(&'\u{E000}')
                .max(&'\u{10FFFF}')
                .build(),
        ))
        .build();

    // 39 non-eol = %x09 / %x20-7F / non-ascii
    let non_eol = Op::Or(
        Cnds::default()
            .push(&Cnd::Pin(0x09 as char))
            .push(&Cnd::RangeIncludesMax(
                RangeIncludesMax::default()
                    .min(&(0x20 as char))
                    .max(&(0x7F as char))
                    .build(),
            ))
            .extend(&non_ascii)
            .build(),
    );

    // 41 comment = comment-start-symbol *non-eol
    // TODO これを条件文として持てないか？ Control を持つ Routine レイヤーを作るか？
    let comment = Expected::default() // "# Comment."
        .routine(
            &Ro::default()
                .push(&Co::Once(Op::One(comment_start_symbol)))
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&non_eol)
                        .min(0)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .build(),
        )
        .build();

    // Digit.
    let digit = Cnd::RangeIncludesMax(RangeIncludesMax::default().min(&'0').max(&'9').build());
    // Alphabet.
    let upper_case = Cnd::RangeIncludesMax(RangeIncludesMax::default().min(&'A').max(&'Z').build());
    let lower_case = Cnd::RangeIncludesMax(RangeIncludesMax::default().min(&'a').max(&'z').build());
    let alpha = Cnds::default().push(&upper_case).push(&lower_case).build();

    let ex1 = Expected::default() // "(wschar)   1"
        .routine(
            &Ro::default()
                .push(&Co::Once(Op::Or(wschar.clone())))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin(' '))))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex2 = Expected::default() // "+(wschar)"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex3 = Expected::default() // "(wschar){5,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(5)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();

    let ex4 = Expected::default() // "(wschar){0,3}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(0)
                        .max_not_included(3)
                        .build(),
                ))
                .push(&Co::Once(Op::One(Cnd::Pin('1'))))
                .build(),
        )
        .build();
    let ex5 = Expected::default() // "(wschar){1,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(wschar.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .push(&Co::Once(Op::One(digit.clone())))
                .build(),
        )
        .build();
    let ex6 = Expected::default() // "(alpha)"
        .routine(&Ro::default().push(&Co::Once(Op::Or(alpha.clone()))).build())
        .build();
    let ex7 = Expected::default() // "(alpha){1,3}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(alpha.clone()))
                        .min(1)
                        .max_not_included(3)
                        .build(),
                ))
                .build(),
        )
        .build();

    let ex8 = Expected::default() // "(alpha){1,}"
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .op(&Op::Or(alpha.clone()))
                        .min(1)
                        .max_not_included(usize::MAX)
                        .build(),
                ))
                .build(),
        )
        .build();

    let ex9 = Expected::default() // "(newline)"
        .routine(
            &Ro::default()
                .push(&Co::Once(Op::Or(newline.clone())))
                .build(),
        )
        .build();

    let unquoted_key = Expected::default() // 'No-1_0' - Unquloted key.
        .routine(
            &Ro::default()
                .push(&Co::Repeat(
                    Repeat::default()
                        .min(1)
                        .max_not_included(usize::MAX)
                        .op(&Op::Or(
                            Cnds::default()
                                .extend(&alpha) // A-Z, a-z.
                                .push(&digit) // 0-9.
                                .push(&Cnd::Pin(0x2D as char)) // -
                                .push(&Cnd::Pin(0x5F as char)) // _
                                .build(),
                        ))
                        .build(),
                ))
                .build(),
        )
        .build();

    assert!(Ma::default().actual(&ac1).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac2).expected(&ex1).build().exec());
    assert!(!Ma::default().actual(&ac3).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex2).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex3).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex4).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex5).build().exec());
    assert!(Ma::default().actual(&ac4).expected(&ex6).build().exec());
    assert!(Ma::default().actual(&ac5).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac6).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac7).expected(&ex8).build().exec());
    assert!(Ma::default().actual(&ac8).expected(&ex9).build().exec());
    // コメントのテスト
    assert!(Ma::default().actual(&ac9).expected(&comment).build().exec());
    // コメントではないもののテスト
    assert!(!Ma::default().actual(&ac5).expected(&comment).build().exec());
    // 'No-1_0' - Unquloted key.
    assert!(Ma::default()
        .actual(&ac10)
        .expected(&unquoted_key)
        .build()
        .exec());
    // Newline is not Unquloted key.
    assert!(!Ma::default()
        .actual(&ac8)
        .expected(&unquoted_key)
        .build()
        .exec());

    println!("Finished.");
}
