//! Create `Expected`.  
//! `Expected` を作成します。  
use crate::{Controls, Expected, ExpectedBuilder};

impl<T> Default for ExpectedBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ExpectedBuilder { items: Vec::new() }
    }
}

impl<T> ExpectedBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `Expected`.  
    //! `Expected` を作成します。  
    pub fn build(&self) -> Expected<T>
    where
        T: std::clone::Clone,
    {
        Expected {
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
