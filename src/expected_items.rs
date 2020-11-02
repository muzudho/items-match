use crate::{Expected, ExpectedItems};

impl<T> Default for ExpectedItems<T> {
    fn default() -> Self {
        ExpectedItems { items: Vec::new() }
    }
}

impl<T> ExpectedItems<T> {
    pub fn get(&self, index: usize) -> Option<&Expected<T>> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Expected<T>> {
        if index < self.items.len() {
            Some(&mut self.items[index])
        } else {
            None
        }
    }
}
