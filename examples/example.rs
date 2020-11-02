// extern crate look_ahead_items;
extern crate rattle_items_match;

use rattle_items_match::RangeContainsMaxBuilder;
use rattle_items_match::{
    ActualItemsBuilder, AnyBuilder, Expected, ExpectedItemsBuilder, Machine, RepeatBuilder,
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
    let wschar = AnyBuilder::default().push(&'\t').push(&' ').build();

    // Digit.
    let digit = RangeContainsMaxBuilder::default()
        .set_min(&'0')
        .set_max(&'9')
        .build();

    let mut expected_items1 = ExpectedItemsBuilder::default()
        .push(&Expected::Any(wschar.clone()))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact('1'))
        .build();

    assert!(Machine::default().matching(&actual_items1, &mut expected_items1));
    assert!(Machine::default().matching(&actual_items2, &mut expected_items1));
    assert!(!Machine::default().matching(&actual_items3, &mut expected_items1));
    let mut expected_items2 = ExpectedItemsBuilder::default()
        .push(&Expected::Repeat(
            RepeatBuilder::default()
                .set_expected(&Expected::Any(wschar.clone()))
                .set_min(1)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Expected::Exact('1'))
        .build();
    let mut expected_items3 = ExpectedItemsBuilder::default()
        .push(&Expected::Repeat(
            RepeatBuilder::default()
                .set_expected(&Expected::Any(wschar.clone()))
                .set_min(5)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Expected::Exact('1'))
        .build();
    let mut expected_items4 = ExpectedItemsBuilder::default()
        .push(&Expected::Repeat(
            RepeatBuilder::default()
                .set_expected(&Expected::Any(wschar.clone()))
                .set_min(0)
                .set_max(3)
                .build(),
        ))
        .push(&Expected::Exact('1'))
        .build();
    let mut expected_items5 = ExpectedItemsBuilder::default()
        .push(&Expected::Repeat(
            RepeatBuilder::default()
                .set_expected(&Expected::Any(wschar.clone()))
                .set_min(1)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Expected::RangeContainsMax(digit))
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
