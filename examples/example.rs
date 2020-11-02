extern crate items_match;
extern crate look_ahead_items;

use items_match::AnyBuilder;
use items_match::{ActualItemsBuilder, ExpectedItem, ExpectedItemsBuilder, Machine};

fn main() {
    println!("Start.");

    // Whitespace characters.
    let wschar = AnyBuilder::default().push(&'\t').push(&' ').build();

    let actual_items = ActualItemsBuilder::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'a')
        .build();

    let expected_items = ExpectedItemsBuilder::default()
        .push(&ExpectedItem::Any(wschar))
        .push(&ExpectedItem::Exact(' '))
        .push(&ExpectedItem::Exact(' '))
        .push(&ExpectedItem::Exact(' '))
        .push(&ExpectedItem::Exact('a'))
        .build();

    let mut machine = Machine::default();
    assert!(machine.matching(&actual_items, &expected_items));
    // assert!(expected_items.matched(&vec!['a', 'b']));

    println!("Finished.");
}
