//! Create `Actual`.  
//! `Actual` を作成します。  
use crate::{Actual, ActualBuilder};

impl<T> Default for ActualBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ActualBuilder { items: Vec::new() }
    }
}

impl<T> ActualBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `Actual`.  
    //! `Actual` を作成します。  
    pub fn build(&self) -> Actual<T>
    where
        T: std::clone::Clone,
    {
        Actual {
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
