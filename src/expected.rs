//! Create `ExpectedVal`.  
//! `ExpectedVal` を作成します。  
use crate::{Controls, Expected, ExpectedVal};

impl<T> Default for Expected<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        Expected { items: Vec::new() }
    }
}

impl<T> Expected<T>
where
    T: std::clone::Clone,
{
    //! Create `ExpectedVal`.  
    //! `ExpectedVal` を作成します。  
    pub fn build(&self) -> ExpectedVal<T>
    where
        T: std::clone::Clone,
    {
        ExpectedVal {
            items: self.items.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &Controls<T>) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}

impl<T> Default for ExpectedVal<T> {
    fn default() -> Self {
        ExpectedVal { items: Vec::new() }
    }
}

impl<T> ExpectedVal<T> {
    pub fn get(&self, index: usize) -> Option<&Controls<T>> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
