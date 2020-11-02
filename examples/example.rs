extern crate rattle_items_match;

use rattle_items_match::Expected;
use rattle_items_match::MachineBuilder;
use rattle_items_match::Quantity;
use rattle_items_match::RangeContainsMaxBuilder;
use rattle_items_match::{
    ActualItemsBuilder, AnyBuilder, Controls, ExpectedItemsBuilder, RepeatBuilder,
};

fn main() {
    println!("Start.");

    let act1_ssss1 = ActualItemsBuilder::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act2_tsss1 = ActualItemsBuilder::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act3_xsss1 = ActualItemsBuilder::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let act4_a = ActualItemsBuilder::default().push(&'A').build();
    let act5_bc = ActualItemsBuilder::default().push(&'B').push(&'C').build();
    let act6_de = ActualItemsBuilder::default().push(&'d').push(&'e').build();
    let act7_fgh = ActualItemsBuilder::default()
        .push(&'f')
        .push(&'g')
        .push(&'h')
        .build();

    // Whitespace characters.
    let wschar = AnyBuilder::default()
        .push(&Expected::Exact('\t'))
        .push(&Expected::Exact(' '))
        .build();

    // Digit.
    let digit = RangeContainsMaxBuilder::default()
        .set_min(&'0')
        .set_max(&'9')
        .build();
    // Alphabet.
    let upper_case = Expected::RangeContainsMax(
        RangeContainsMaxBuilder::default()
            .set_min(&'A')
            .set_max(&'Z')
            .build(),
    );
    let lower_case = Expected::RangeContainsMax(
        RangeContainsMaxBuilder::default()
            .set_min(&'a')
            .set_max(&'z')
            .build(),
    );
    let alpha = AnyBuilder::default()
        .push(&upper_case)
        .push(&lower_case)
        .build();

    let expected1_wsss1 = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(wschar.clone())))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();

    let expected2_ws1max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let expected3_ws5max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let expected4_ws03 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let expected5_ws1max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::RangeContainsMax(
            digit,
        ))))
        .build();
    let expected6_alpha = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(alpha.clone())))
        .build();
    let expected7_alpha1to3 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let expected8_alpha1to_max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act2_tsss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act3_xsss1)
        .set_expected_items(&expected1_wsss1)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected2_ws1max)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected3_ws5max)
        .build()
        .matching());
    assert!(!MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected4_ws03)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act1_ssss1)
        .set_expected_items(&expected5_ws1max)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act4_a)
        .set_expected_items(&expected6_alpha)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act5_bc)
        .set_expected_items(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act6_de)
        .set_expected_items(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(MachineBuilder::default()
        .set_actual_items(&act7_fgh)
        .set_expected_items(&expected8_alpha1to_max)
        .build()
        .matching());
    println!("Finished.");
}
