extern crate rattle_items_match;

use rattle_items_match::{
    Actual, Any, Controls as Co, Element as El, Expected, Machine, Quantity as Qu,
    RangeIncludesMax, Repeat,
};

fn main() {
    println!("Start.");

    let ac1_ssss1 = Actual::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac2_tsss1 = Actual::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac3_xsss1 = Actual::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let ac4_a = Actual::default().push(&'A').build();
    let ac5_bc = Actual::default().push(&'B').push(&'C').build();
    let ac6_de = Actual::default().push(&'d').push(&'e').build();
    let ac7_fgh = Actual::default().push(&'f').push(&'g').push(&'h').build();

    // Whitespace characters.
    let wschar = Any::default()
        .push(&El::Pin('\t'))
        .push(&El::Pin(' '))
        .build();

    /*
    // Newline.
    let wschar = Any::default()
        .push(&El::Pin(0x20))
        .push(&El::Pin(' '))
        .build();
    */

    // Digit.
    let digit = RangeIncludesMax::default().min(&'0').max(&'9').build();
    // Alphabet.
    let upper_case = El::RangeIncludesMax(RangeIncludesMax::default().min(&'A').max(&'Z').build());
    let lower_case = El::RangeIncludesMax(RangeIncludesMax::default().min(&'a').max(&'z').build());
    let alpha = Any::default().push(&upper_case).push(&lower_case).build();

    // #
    let comment_start_symbol = El::Pin('#');

    let ex1_wsss1 = Expected::default()
        .push(&Co::Once(Qu::Any(wschar.clone())))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();

    let ex2_ws1max = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex3_ws5max = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(wschar.clone()))
                .min(5)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex4_ws03 = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(wschar.clone()))
                .min(0)
                .max_not_included(3)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex5_ws1max = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::RangeIncludesMax(digit))))
        .build();
    let ex6_alpha = Expected::default()
        .push(&Co::Once(Qu::Any(alpha.clone())))
        .build();
    let ex7_alpha1to3 = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(alpha.clone()))
                .min(1)
                .max_not_included(3)
                .build(),
        ))
        .build();
    let ex8_alpha1to_max = Expected::default()
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Any(alpha.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    assert!(Machine::default()
        .actual(&ac1_ssss1)
        .expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac2_tsss1)
        .expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(!Machine::default()
        .actual(&ac3_xsss1)
        .expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac1_ssss1)
        .expected(&ex2_ws1max)
        .build()
        .matching());
    assert!(!Machine::default()
        .actual(&ac1_ssss1)
        .expected(&ex3_ws5max)
        .build()
        .matching());
    assert!(!Machine::default()
        .actual(&ac1_ssss1)
        .expected(&ex4_ws03)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac1_ssss1)
        .expected(&ex5_ws1max)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac4_a)
        .expected(&ex6_alpha)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac5_bc)
        .expected(&ex7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac6_de)
        .expected(&ex7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .actual(&ac7_fgh)
        .expected(&ex8_alpha1to_max)
        .build()
        .matching());
    println!("Finished.");
}
