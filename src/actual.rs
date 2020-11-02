use crate::Actual;

impl<T> Default for Actual<T> {
    fn default() -> Self {
        Actual { items: Vec::new() }
    }
}

impl<T> Actual<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }

    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
