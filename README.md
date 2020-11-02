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
extern crate items_match;
extern crate look_ahead_items;

use items_match::AnyBuilder;
use items_match::{ActualItemsBuilder, Expected, ExpectedItemsBuilder, Machine};

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
```
