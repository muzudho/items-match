use crate::{Controls, Expected};

impl<T> Default for Expected<T> {
    fn default() -> Self {
        Expected { items: Vec::new() }
    }
}

impl<T> Expected<T> {
    pub fn get(&self, index: usize) -> Option<&Controls<T>> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
