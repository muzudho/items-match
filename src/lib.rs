pub mod actual_items;
pub mod actual_items_builder;
pub mod any;
pub mod any_builder;
pub mod expected_items;
pub mod expected_items_builder;
pub mod machine;

pub struct Machine {
    index: usize,
}

pub struct ActualItemsBuilder<T> {
    items: Vec<T>,
}

pub struct ActualItems<T> {
    items: Vec<T>,
}

pub struct ExpectedItemsBuilder<T> {
    items: Vec<ExpectedItem<T>>,
}

pub struct ExpectedItems<T> {
    items: Vec<ExpectedItem<T>>,
}

#[derive(Clone)]
pub enum ExpectedItem<T> {
    Exact(T),
    Any(Any<T>),
}

pub struct AnyBuilder<T> {
    items: Vec<T>,
}

#[derive(Clone)]
pub struct Any<T> {
    items: Vec<T>,
}
