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

use rattle_items_match::Element;
use rattle_items_match::Machine;
use rattle_items_match::Quantity;
use rattle_items_match::RangeIncludesMaxBuilder;
use rattle_items_match::{
    Actual, AnyBuilder, Controls, ExpectedBuilder, RepeatBuilder,
};

fn main() {
    println!("Start.");

    let act1_ssss1 = Actual::default()
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act2_tsss1 = Actual::default()
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let act3_xsss1 = Actual::default()
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let act4_a = Actual::default().push(&'A').build();
    let act5_bc = Actual::default().push(&'B').push(&'C').build();
    let act6_de = Actual::default().push(&'d').push(&'e').build();
    let act7_fgh = Actual::default()
        .push(&'f')
        .push(&'g')
        .push(&'h')
        .build();

    // Whitespace characters.
    let wschar = AnyBuilder::default()
        .push(&Element::Exact('\t'))
        .push(&Element::Exact(' '))
        .build();

    // Digit.
    let digit = RangeIncludesMaxBuilder::default()
        .set_min(&'0')
        .set_max(&'9')
        .build();
    // Alphabet.
    let upper_case = Element::RangeIncludesMax(
        RangeIncludesMaxBuilder::default()
            .set_min(&'A')
            .set_max(&'Z')
            .build(),
    );
    let lower_case = Element::RangeIncludesMax(
        RangeIncludesMaxBuilder::default()
            .set_min(&'a')
            .set_max(&'z')
            .build(),
    );
    let alpha = AnyBuilder::default()
        .push(&upper_case)
        .push(&lower_case)
        .build();

    let expected1_wsss1 = ExpectedBuilder::default()
        .push(&Controls::Once(Quantity::Any(wschar.clone())))
        .push(&Controls::Once(Quantity::One(Element::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Element::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Element::Exact(' '))))
        .push(&Controls::Once(Quantity::One(Element::Exact('1'))))
        .build();

    let expected2_ws1max = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Element::Exact('1'))))
        .build();
    let expected3_ws5max = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(5)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Element::Exact('1'))))
        .build();
    let expected4_ws03 = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(0)
                .set_max_not_included(3)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Element::Exact('1'))))
        .build();
    let expected5_ws1max = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(wschar.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Controls::Once(Quantity::One(Element::RangeIncludesMax(
            digit,
        ))))
        .build();
    let expected6_alpha = ExpectedBuilder::default()
        .push(&Controls::Once(Quantity::Any(alpha.clone())))
        .build();
    let expected7_alpha1to3 = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(3)
                .build(),
        ))
        .build();
    let expected8_alpha1to_max = ExpectedBuilder::default()
        .push(&Controls::Repeat(
            RepeatBuilder::default()
                .set_quantity(&Quantity::Any(alpha.clone()))
                .set_min(1)
                .set_max_not_included(usize::MAX)
                .build(),
        ))
        .build();

    assert!(Machine::default()
        .set_actual(&act1_ssss1)
        .set_expected(&expected1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act2_tsss1)
        .set_expected(&expected1_wsss1)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&act3_xsss1)
        .set_expected(&expected1_wsss1)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act1_ssss1)
        .set_expected(&expected2_ws1max)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&act1_ssss1)
        .set_expected(&expected3_ws5max)
        .build()
        .matching());
    assert!(!Machine::default()
        .set_actual(&act1_ssss1)
        .set_expected(&expected4_ws03)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act1_ssss1)
        .set_expected(&expected5_ws1max)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act4_a)
        .set_expected(&expected6_alpha)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act5_bc)
        .set_expected(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act6_de)
        .set_expected(&expected7_alpha1to3)
        .build()
        .matching());
    assert!(Machine::default()
        .set_actual(&act7_fgh)
        .set_expected(&expected8_alpha1to_max)
        .build()
        .matching());
    println!("Finished.");
}
```
