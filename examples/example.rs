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
        .push(&Controls::Once(Quantity::One(' ')))
        .push(&Controls::Once(Quantity::One(' ')))
        .push(&Controls::Once(Quantity::One(' ')))
        .push(&Controls::Once(Quantity::One('1')))
        .build();

    assert!(Machine::default().matching(&actual_items1, &mut expected_items1));
    assert!(Machine::default().matching(&actual_items2, &mut expected_items1));
    assert!(!Machine::default().matching(&actual_items3, &mut expected_items1));
    let mut expected_items2 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_expected(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One('1')))
        .build();
    let mut expected_items3 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_expected(&Quantity::Any(wschar.clone()))
                .set_min(5)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One('1')))
        .build();
    let mut expected_items4 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_expected(&Quantity::Any(wschar.clone()))
                .set_min(0)
                .set_max(3)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One('1')))
        .build();
    let mut expected_items5 = ExpectedItemsBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_expected(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::RangeContainsMax(digit)))
        .build();

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

    println!("Finished.");
}
