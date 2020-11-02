use crate::ActualItems;

impl<T> Default for ActualItems<T> {
    fn default() -> Self {
        ActualItems { items: Vec::new() }
    }
}

impl<T> ActualItems<T> {
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
}
