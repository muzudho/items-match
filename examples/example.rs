extern crate look_ahead_items;
extern crate rattle_items_match;

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

    let mut expected_items1 = ExpectedItemsBuilder::default()
        .push(&Expected::Any(wschar.clone()))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact(' '))
        .push(&Expected::Exact('a'))
        .build();

    let mut machine = Machine::default();
    assert!(machine.matching(&actual_items1, &mut expected_items1));
    assert!(machine.matching(&actual_items2, &mut expected_items1));
    assert!(!machine.matching(&actual_items3, &mut expected_items1));

    let mut expected_items2 = ExpectedItemsBuilder::default()
        .push(&Expected::Repeat(
            RepeatBuilder::default()
                .set_expected(&Expected::Any(wschar.clone()))
                .set_min(1)
                .set_max(usize::MAX)
                .build(),
        ))
        .push(&Expected::Exact('a'))
        .build();

    assert!(machine.matching(&actual_items1, &mut expected_items2));

    println!("Finished.");
}
