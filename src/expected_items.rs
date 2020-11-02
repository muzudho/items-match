use crate::ExpectedItems;

impl<T> Default for ExpectedItems<T> {
    fn default() -> Self {
        ExpectedItems { items: Vec::new() }
    }
}

impl<T> ExpectedItems<T> {
    pub fn matched(&self, actual: &T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        if 0 < self.items.len() && self.items[0] == *actual {
            true
        } else {
            false
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
