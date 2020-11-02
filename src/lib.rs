//! Matching is not limited to character strings. I'm trying to make a game AI.  
//! 文字列に限らないマッチングです。 ゲームAIを作ろうとしています。  

// Publish:
//
// (1) `cargo test`
// (2) `cargo run --example example`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Update `README.md`.
// (6) Version up on Cargo.toml.
// (7) `cargo doc --open`
// (8) Comit to Git-hub.
// (9) `cargo publish --dry-run`
// (10) `cargo publish`

pub mod actual_items;
pub mod actual_items_builder;
pub mod any;
pub mod any_builder;
pub mod expected;
pub mod expected_builder;
pub mod machine;
pub mod machine_builder;
pub mod machine_state;
pub mod range_contains_max;
pub mod range_contains_max_builder;
pub mod repeat;
pub mod repeat_builder;

pub struct MachineBuilder<T> {
    actual_items: Option<ActualItems<T>>,
    expected_items: Option<Expected<T>>,
}

pub struct MachineState {
    actual_index: usize,
    expected_index: usize,
    is_final: bool,
    matched_length_in_repeat: usize,
}

pub struct Machine<T> {
    actual_items: ActualItems<T>,
    expected_items: Expected<T>,
}

pub struct ActualItemsBuilder<T> {
    items: Vec<T>,
}

#[derive(Clone)]
pub struct ActualItems<T> {
    items: Vec<T>,
}

pub struct ExpectedBuilder<T> {
    items: Vec<Controls<T>>,
}

#[derive(Clone)]
pub struct Expected<T> {
    items: Vec<Controls<T>>,
}

/// Controls item.  
/// 制御項目。  
#[derive(Clone)]
pub enum Controls<T> {
    Once(Quantity<T>),
    Repeat(Repeat<T>),
}
/// Quantity.  
/// 量。  
#[derive(Clone)]
pub enum Quantity<T> {
    One(Element<T>),
    Any(Any<T>),
}
/// Element.
/// 期待値。
#[derive(Clone)]
pub enum Element<T> {
    Exact(T),
    RangeContainsMax(RangeContainsMax<T>),
}

pub struct RangeContainsMaxBuilder<T> {
    min: Option<T>,
    max: Option<T>,
}

/// Specify by range. Includes maximum value.  
/// 範囲で指定。最大値を含みます。  
#[derive(Clone)]
pub struct RangeContainsMax<T> {
    min: Option<T>,
    max: Option<T>,
}

pub struct AnyBuilder<T> {
    items: Vec<Element<T>>,
}

#[derive(Clone)]
pub struct Any<T> {
    items: Vec<Element<T>>,
}

pub struct RepeatBuilder<T> {
    quantity: Option<Box<Quantity<T>>>,
    min: usize,
    max_not_included: usize,
}

#[derive(Clone)]
pub struct Repeat<T> {
    quantity: Box<Quantity<T>>,
    min: usize,
    max_not_included: usize,
}

pub enum MatchingResult {
    Ongoing,
    Matched,
    NotMatch,
}
