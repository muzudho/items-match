extern crate rattle_items_match;

use rattle_items_match::{
    ActualBuilder as Actual, Controls as Co, ExpectedBuilder as Expected, MachineBuilder as Ma,
    OrOperand as Nd, OrOperandsBuilder as Nds, OrOperator as Or, RangeIncludesMax, Repeat,
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
    let ac9 = Actual::default() // '# Comment.'
        .push(&'#')
        .push(&' ')
        .push(&'C')
        .push(&'o')
        .push(&'m')
        .push(&'m')
        .push(&'e')
        .push(&'n')
        .push(&'t')
        .push(&'.')
        .build();

    // Whitespace characters.
    let wschar = Nds::default()
        .push(&Nd::Pin('\t'))
        .push(&Nd::Pin(' '))
        .build();

    // Newline.
    let newline = Nds::default()
        .push(&Nd::Pin('\n')) // LF
        .push(&Nd::Seq(vec!['\r', '\n'])) // CR LF
        .build();

    // Digit.
    let digit = RangeIncludesMax::default().min(&'0').max(&'9').build();
    // Alphabet.
    let upper_case = Nd::RangeIncludesMax(RangeIncludesMax::default().min(&'A').max(&'Z').build());
    let lower_case = Nd::RangeIncludesMax(RangeIncludesMax::default().min(&'a').max(&'z').build());
    let alpha = Nds::default().push(&upper_case).push(&lower_case).build();

    let comment_start_symbol = Nd::Pin('#'); // #
    let non_ascii = Or::Any(
        Nds::default()
            .push(&Nd::RangeIncludesMax(
                RangeIncludesMax::default()
                    .min(&(0x80 as char))
                    .max(&'\u{D7FF}')
                    .build(),
            ))
            .build(),
    );
    let non_eol = Or::Any(
        Nds::default()
            .push(&Nd::Pin(0x09 as char))
            .push(&Nd::RangeIncludesMax(
                RangeIncludesMax::default()
                    .min(&(0x20 as char))
                    .max(&(0x7F as char))
                    .build(),
            ))
            // TODO push non_ascii
            .build(),
    );

    let ex1 = Expected::default() // "(wschar)   1"
        .push(&Co::Once(Or::Any(wschar.clone())))
        .push(&Co::Once(Or::One(Nd::Pin(' '))))
        .push(&Co::Once(Or::One(Nd::Pin(' '))))
        .push(&Co::Once(Or::One(Nd::Pin(' '))))
        .push(&Co::Once(Or::One(Nd::Pin('1'))))
        .build();

    let ex2 = Expected::default() // "+(wschar)"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Or::One(Nd::Pin('1'))))
        .build();
    let ex3 = Expected::default() // "(wschar){5,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(wschar.clone()))
                .min(5)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Or::One(Nd::Pin('1'))))
        .build();
    let ex4 = Expected::default() // "(wschar){0,3}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(wschar.clone()))
                .min(0)
                .max_not_included(3)
                .build(),
        ))
        .push(&Co::Once(Or::One(Nd::Pin('1'))))
        .build();
    let ex5 = Expected::default() // "(wschar){1,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Or::One(Nd::RangeIncludesMax(digit))))
        .build();
    let ex6 = Expected::default() // "(alpha)"
        .push(&Co::Once(Or::Any(alpha.clone())))
        .build();
    let ex7 = Expected::default() // "(alpha){1,3}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(alpha.clone()))
                .min(1)
                .max_not_included(3)
                .build(),
        ))
        .build();
    let ex8 = Expected::default() // "(alpha){1,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Or::Any(alpha.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .build();
    let ex9 = Expected::default() // "(newline)"
        .push(&Co::Once(Or::Any(newline.clone())))
        .build();
    let comment = Expected::default() // "# Comment."
        .push(&Co::Once(Or::One(comment_start_symbol)))
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&non_eol)
                .min(0)
                .max_not_included(usize::MAX)
                .build(),
        ))
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
    assert!(Ma::default().actual(&ac9).expected(&comment).build().exec());
    println!("Finished.");
}
