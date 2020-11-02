# rattle-items-match

Matching is not limited to character strings. I'm trying to make a game AI.  
文字列に限らないマッチングです。 ゲームAIを作ろうとしています。  

Unstable version. It's a tryal and error process. Specifications will change.  
不安定版。 試行錯誤中です。 仕様はコロコロ変わるでしょう。  

I named it rattle because it's a lot of detailed work.  
細かい作業がいっぱいなのでrattleという名前にしました。  

## Run

Take a look at the repository.  
リポジトリを見てください。  

```shell
cargo run --example example
```

## Specification (仕様)

The specifications will gradually solidify.  
仕様は少しずつ固めていきます。  

You can think that you can't do anything that isn't written here.  
ここに書かれていないことは何もできないと思ってもらって構いません。  

./examples/example.rs:  

```rust
extern crate rattle_items_match;

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
    let wschar = AnyBuilder::default().push(&'\t').push(&' ').build();

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
```
