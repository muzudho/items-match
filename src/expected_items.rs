use crate::{ExpectedItem, ExpectedItems};

impl<T> Default for ExpectedItems<T> {
    fn default() -> Self {
        ExpectedItems { items: Vec::new() }
    }
}

impl<T> ExpectedItems<T> {
    /*
    pub fn matched(&self, actual: &T) -> bool
    where
        T: std::cmp::PartialEq,
    {
        if 0 < self.items.len() {
            match self.items[0] {
                ExpectedItem::Any(any) => {
                    for exp in any.items {
                        if *exp == *actual {
                            return true;
                        }
                    }
                    return false;
                }
                ExpectedItem::Exact(exp) => {
                    if *exp == *actual {
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }
    */

    pub fn get(&self, index: usize) -> Option<&ExpectedItem<T>> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
