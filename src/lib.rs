pub mod actual_items;
pub mod actual_items_builder;
pub mod expected_items;
pub mod expected_items_builder;
pub mod machine;

pub struct Machine {
    index: usize,
}

pub struct ExpectedItemsBuilder<T> {
    items: Vec<T>,
}

pub struct ActualItemsBuilder<T> {
    items: Vec<T>,
}

pub struct ExpectedItems<T> {
    items: Vec<T>,
}

pub struct ActualItems<T> {
    items: Vec<T>,
}
