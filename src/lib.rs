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

pub mod actual;
pub mod condition;
pub mod expected;
pub mod machine;
pub mod range_includes_max;
pub mod repeat;

pub struct MachineBuilder<T> {
    actual: Option<ActualVal<T>>,
    expected: Option<ExpectedVal<T>>,
}

pub struct MachineState {
    actual_index: usize,
    expected_index: usize,
    is_final: bool,
    matched_length_in_repeat: usize,
    matched_length_in_seq: usize,
}

pub struct MachineVal<T> {
    actual: ActualVal<T>,
    expected: ExpectedVal<T>,
}

pub struct ActualBuilder<T> {
    items: Vec<T>,
}

#[derive(Clone)]
pub struct ActualVal<T> {
    items: Vec<T>,
}

pub struct ExpectedBuilder<T> {
    items: Vec<Control<T>>,
    // TODO routine: Routine<T>,
}

#[derive(Clone)]
pub struct ExpectedVal<T> {
    items: Vec<Control<T>>,
    // TODO routine: Routine<T>,
}

#[derive(Clone)]
pub struct Routine<T> {
    controls: Vec<Control<T>>,
}

/// Control.  
/// 制御。  
#[derive(Clone)]
pub enum Control<T> {
    Once(Operator<T>),
    Repeat(RepeatVal<T>),
}
#[derive(Clone)]
pub enum Operator<T> {
    /// １つしかなければ、これが簡便。  
    One(Condition<T>),
    /// OR operator.  
    /// OR演算子(||)で、条件式をつないでいることに相当します。  
    ///
    /// This is for multinomial operators.  
    /// 多項演算子にするならこれ。どれか１つでもマッチすれば、マッチ。  
    Or(ConditionsVal<T>),
}
/// Condition. Logical operator not included.  
/// 条件式。この並びに論理演算子は含みません。  
#[derive(Clone)]
pub enum Condition<T> {
    /// 1つだけのもの。  
    Pin(T),
    /// Sequence.  
    /// 連続のもの。  
    Seq(Vec<T>),
    /// 範囲で指定。  
    RangeIncludesMax(RangeIncludesMaxVal<T>),
}

pub struct RangeIncludesMax<T> {
    min: Option<T>,
    max: Option<T>,
}

/// Specify by range. Includes maximum value.  
/// 範囲で指定。最大値を含みます。  
#[derive(Clone)]
pub struct RangeIncludesMaxVal<T> {
    min: Option<T>,
    max: Option<T>,
}

pub struct ConditionsBuilder<T> {
    conditions: Vec<Condition<T>>,
}

#[derive(Clone)]
pub struct ConditionsVal<T> {
    conditions: Vec<Condition<T>>,
}

pub struct Repeat<T> {
    /// Operator.  
    /// 演算子。
    op: Option<Box<Operator<T>>>,
    min: usize,
    max_not_included: usize,
}

#[derive(Clone)]
pub struct RepeatVal<T> {
    op: Box<Operator<T>>,
    min: usize,
    max_not_included: usize,
}

pub enum MatchingResult {
    Ongoing,
    Matched,
    NotMatch,
}
