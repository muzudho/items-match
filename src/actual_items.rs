use crate::ActualItems;

impl<T> Default for ActualItems<T> {
    fn default() -> Self {
        ActualItems { items: Vec::new() }
    }
}

impl<T> ActualItems<T> {
    /*
    pub fn matched(&self, actual: &T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        let index = 0;
        if index < self.items.len() && self.items[index] == *actual {
            true
        } else {
            false
        }
    }
    */

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
