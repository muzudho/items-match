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

use rattle_items_match::{
    ActualBuilder, OrOperandsBuilder, Controls as Co, Condition as El, ExpectedBuilder, MachineBuilder as Ma, Operator as Qu,
    RangeIncludesMax, Repeat,
};

fn main() {
    println!("Start.");

    // `Actual` is sequence only.
    // 比較対象値は シーケンスのみです。
    let ac1 = ActualBuilder::default() // "    1"
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac2 = ActualBuilder::default() // "\t   1"
        .push(&'\t')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();

    let ac3 = ActualBuilder::default() // 'x   1'
        .push(&'x')
        .push(&' ')
        .push(&' ')
        .push(&' ')
        .push(&'1')
        .build();
    let ac4 = ActualBuilder::default().push(&'A').build(); // 'A'
    let ac5 = ActualBuilder::default().push(&'B').push(&'C').build(); // 'BC'
    let ac6 = ActualBuilder::default().push(&'d').push(&'e').build(); // 'de'
    let ac7 = ActualBuilder::default().push(&'f').push(&'g').push(&'h').build(); // 'fgh'
    let ac8 = ActualBuilder::default().push(&'\r').push(&'\n').build(); // "\r\n"

    // Whitespace characters.
    let wschar = OrOperandsBuilder::default()
        .push(&El::Pin('\t'))
        .push(&El::Pin(' '))
        .build();

    // Newline.
    let newline = OrOperandsBuilder::default()
        .push(&El::Pin('\n')) // LF
        .push(&El::Seq(vec!['\r', '\n'])) // CR LF
        .build();

    // Digit.
    let digit = RangeIncludesMax::default().min(&'0').max(&'9').build();
    // Alphabet.
    let upper_case = El::RangeIncludesMax(RangeIncludesMax::default().min(&'A').max(&'Z').build());
    let lower_case = El::RangeIncludesMax(RangeIncludesMax::default().min(&'a').max(&'z').build());
    let alpha = OrOperandsBuilder::default().push(&upper_case).push(&lower_case).build();

    // #
    // TODO let comment_start_symbol = El::Pin('#');

    let ex1 = ExpectedBuilder::default() // "(wschar)   1"
        .push(&Co::Once(Qu::Or(wschar.clone())))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin(' '))))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();

    let ex2 = ExpectedBuilder::default() // "+(wschar)"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex3 = ExpectedBuilder::default() // "(wschar){5,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(wschar.clone()))
                .min(5)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex4 = ExpectedBuilder::default() // "(wschar){0,3}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(wschar.clone()))
                .min(0)
                .max_not_included(3)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::Pin('1'))))
        .build();
    let ex5 = ExpectedBuilder::default() // "(wschar){1,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(wschar.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .push(&Co::Once(Qu::One(El::RangeIncludesMax(digit))))
        .build();
    let ex6 = ExpectedBuilder::default() // "(alpha)"
        .push(&Co::Once(Qu::Or(alpha.clone())))
        .build();
    let ex7 = ExpectedBuilder::default() // "(alpha){1,3}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(alpha.clone()))
                .min(1)
                .max_not_included(3)
                .build(),
        ))
        .build();
    let ex8 = ExpectedBuilder::default() // "(alpha){1,}"
        .push(&Co::Repeat(
            Repeat::default()
                .quantity(&Qu::Or(alpha.clone()))
                .min(1)
                .max_not_included(usize::MAX)
                .build(),
        ))
        .build();
    let ex9 = ExpectedBuilder::default() // "(newline)"
        .push(&Co::Once(Qu::Or(newline.clone())))
        .build();

    assert!(Ma::default().actual(&ac1).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac2).expected(&ex1).build().exec());
    assert!(!Ma::default().actual(&ac3).expected(&ex1).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex2).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex3).build().exec());
    assert!(!Ma::default().actual(&ac1).expected(&ex4).build().exec());
    assert!(Ma::default().actual(&ac1).expected(&ex5).build().exec());
    assert!(Ma::default().actual(&ac4).expected(&ex6).build().exec());
    assert!(Ma::default().actual(&ac5).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac6).expected(&ex7).build().exec());
    assert!(Ma::default().actual(&ac7).expected(&ex8).build().exec());
    assert!(Ma::default().actual(&ac8).expected(&ex9).build().exec());
    println!("Finished.");
}
```
