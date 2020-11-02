//! Create `ActualItems`.  
//! `ActualItems` を作成します。  
use crate::{ActualItems, ActualItemsBuilder};

impl<T> Default for ActualItemsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ActualItemsBuilder { items: Vec::new() }
    }
}

impl<T> ActualItemsBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `ActualItems`.  
    //! `ActualItems` を作成します。  
    pub fn build(&self) -> ActualItems<T>
    where
        T: std::clone::Clone,
    {
        ActualItems {
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
