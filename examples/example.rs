extern crate rattle_items_match;

use rattle_items_match::Expected;
use rattle_items_match::Quantity;
use rattle_items_match::RangeContainsMaxBuilder;
use rattle_items_match::{
    ActualItemsBuilder, AnyBuilder, Controls, ExpectedItemsBuilder, Machine, RepeatBuilder,
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

    let mut expected1_wsss1 = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(wschar.clone())))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();

    let mut expected2_ws1max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected3_ws5max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected4_ws03 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected5_ws1max = ExpectedItemsBuilder::default()
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
    let mut expected6_alpha = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(alpha.clone())))
        .build();
    let mut expected7_alpha1to3 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let mut expected8_alpha1to3 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let mut expected9_alpha1to_max = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    //*
    assert!(Machine::default().matching(&act1_ssss1, &mut expected1_wsss1));
    assert!(Machine::default().matching(&act2_tsss1, &mut expected1_wsss1));
    assert!(!Machine::default().matching(&act3_xsss1, &mut expected1_wsss1));
    // */
    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act1_ssss1, &mut expected2_ws1max);
        // println!("(trace.84) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act1_ssss1, &mut expected3_ws5max);
        // println!("(trace.91) machine={} matched={}", machine, matched);
        assert!(!matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act1_ssss1, &mut expected4_ws03);
        // println!("(trace.99) machine={} matched={}", machine, matched);
        assert!(!matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act1_ssss1, &mut expected5_ws1max);
        assert!(matched);
    }
    // */
    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act4_a, &mut expected6_alpha);
        // println!("(trace.162) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    // */
    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act5_bc, &mut expected7_alpha1to3);
        // println!("(trace.191) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    // */
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act6_de, &mut expected8_alpha1to3);
        // println!("(trace.199) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&act7_fgh, &mut expected9_alpha1to_max);
        // println!("(trace.196) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    println!("Finished.");
}
