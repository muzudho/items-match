extern crate rattle_items_match;

use rattle_items_match::{
    Actual, AnyBuilder, Controls as Co, Element as El, ExpectedBuilder, Machine, Quantity as Qu,
    RangeIncludesMaxBuilder, RepeatBuilder,
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
    let wschar = AnyBuilder::default()
        .push(&El::Exact('\t'))
        .push(&El::Exact(' '))
        .build();

    // Digit.
    let digit = RangeIncludesMaxBuilder::default()
        .set_min(&'0')
        .set_max(&'9')
        .build();
    // Alphabet.
    let upper_case = El::RangeIncludesMax(
        RangeIncludesMaxBuilder::default()
            .set_min(&'A')
            .set_max(&'Z')
            .build(),
    );
    let lower_case = El::RangeIncludesMax(
        RangeIncludesMaxBuilder::default()
            .set_min(&'a')
            .set_max(&'z')
            .build(),
    );
    let alpha = AnyBuilder::default()
        .push(&upper_case)
        .push(&lower_case)
        .build();

    let ex1_wsss1 = ExpectedBuilder::default()
        .push(&Co::Once(Qu::Any(wschar.clone())))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact(' '))))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();

    let ex2_ws1max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let ex3_ws5max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let ex4_ws03 = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Exact('1'))))
        .build();
    let ex5_ws1max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::RangeIncludesMax(digit))))
        .build();
    let ex6_alpha = ExpectedBuilder::default()
        .push(&Co::Once(Qu::Any(alpha.clone())))
        .build();
    let ex7_alpha1to3 = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let ex8_alpha1to_max = ExpectedBuilder::default()
        .push(&Co::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Qu::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    assert!(Machine::default()
        .set_actual(&ac1_ssss1)
        .set_expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac2_tsss1)
        .set_expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&ac3_xsss1)
        .set_expected(&ex1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac1_ssss1)
        .set_expected(&ex2_ws1max)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&ac1_ssss1)
        .set_expected(&ex3_ws5max)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&ac1_ssss1)
        .set_expected(&ex4_ws03)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac1_ssss1)
        .set_expected(&ex5_ws1max)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac4_a)
        .set_expected(&ex6_alpha)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac5_bc)
        .set_expected(&ex7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac6_de)
        .set_expected(&ex7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&ac7_fgh)
        .set_expected(&ex8_alpha1to_max)
        .build()
        .matching());
    println!("Finished.");
}
