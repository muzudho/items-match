extern crate rattle_items_match;

use rattle_items_match::Expected;
use rattle_items_match::Quantity;
use rattle_items_match::RangeContainsMaxBuilder;
use rattle_items_match::{
    ActualItemsBuilder, AnyBuilder, Controls, ExpectedItemsBuilder, Machine, RepeatBuilder,
};

fn main() {
    println!("Start.");

    let actual_items1 = ActualItemsBuilder::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let actual_items2 = ActualItemsBuilder::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let actual_items3 = ActualItemsBuilder::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
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

    let mut expected_items1 = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(wschar.clone())))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();

    //*
    assert!(Machine::default().matching(&actual_items1, &mut expected_items1));
    assert!(Machine::default().matching(&actual_items2, &mut expected_items1));
    assert!(!Machine::default().matching(&actual_items3, &mut expected_items1));
    // */
    let mut expected_items2 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected_items3 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected_items4 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Expected::Exact('1'))))
        .build();
    let mut expected_items5 = ExpectedItemsBuilder::default()
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

    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items1, &mut expected_items2);
        // println!("(trace.84) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items1, &mut expected_items3);
        // println!("(trace.91) machine={} matched={}", machine, matched);
        assert!(!matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items1, &mut expected_items4);
        // println!("(trace.99) machine={} matched={}", machine, matched);
        assert!(!matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items1, &mut expected_items5);
        assert!(matched);
    }
    // */
    let actual_items4 = ActualItemsBuilder::default().push(&'A').build();
    let actual_items5 = ActualItemsBuilder::default().push(&'A').push(&'B').build();
    let actual_items6 = ActualItemsBuilder::default().push(&'a').push(&'b').build();
    let actual_items7 = ActualItemsBuilder::default()
        .push(&'a')
        .push(&'b')
        .push(&'c')
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

    let mut expected_items6 = ExpectedItemsBuilder::default()
        .push(&Controls::Once(Quantity::Any(alpha.clone())))
        .build();
    let mut expected_items7 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let mut expected_items8 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items4, &mut expected_items6);
        println!("(trace.162) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    // */
    //*
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items5, &mut expected_items7);
        println!("(trace.191) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    // */
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items6, &mut expected_items7);
        println!("(trace.199) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    {
        let mut machine = Machine::default();
        let matched = machine.matching(&actual_items7, &mut expected_items8);
        println!("(trace.196) machine={} matched={}", machine, matched);
        assert!(matched);
    }
    println!("Finished.");
}
