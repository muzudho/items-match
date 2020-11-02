//! Create `ExpectedItems`.  
//! `ExpectedItems` を作成します。  
use crate::{ExpectedItems, ExpectedItemsBuilder};

impl<T> Default for ExpectedItemsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ExpectedItemsBuilder { items: Vec::new() }
    }
}

impl<T> ExpectedItemsBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `ExpectedItems`.  
    //! `ExpectedItems` を作成します。  
    pub fn build(&self) -> ExpectedItems<T>
    where
        T: std::clone::Clone,
    {
        ExpectedItems {
            items: self.items.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}
