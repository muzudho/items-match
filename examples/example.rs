extern crate look_ahead_items;
extern crate rattle_items_match;

use rattle_items_match::AnyBuilder;
use rattle_items_match::{ActualItemsBuilder, Expected, ExpectedItemsBuilder, Machine};

fn main() {
    println!("Start.");

    let actual_items1 = ActualItemsBuilder::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'a')
        .build();

    let actual_items2 = ActualItemsBuilder::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'a')
        .build();

    let actual_items3 = ActualItemsBuilder::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'a')
        .build();

    // Whitespace characters.
    let wschar = AnyBuilder::default().push(&'\t').push(&' ').build();

    let expected_items = ExpectedItemsBuilder::default()
        .push(&Expected::Any(wschar))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact('a'))
        .build();

    let mut machine = Machine::default();
    assert!(machine.matching(&actual_items1, &expected_items));
    assert!(machine.matching(&actual_items2, &expected_items));
    assert!(!machine.matching(&actual_items3, &expected_items));

    println!("Finished.");
}
