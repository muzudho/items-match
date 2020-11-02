//! Create `Any`.  
//! `Any` を作成します。  
use crate::Element;
use crate::{Any, AnyBuilder};

impl<T> Default for AnyBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        AnyBuilder { items: Vec::new() }
    }
}

impl<T> AnyBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `Any`.  
    //! `Any` を作成します。  
    pub fn build(&self) -> Any<T>
    where
        T: std::clone::Clone,
    {
        Any {
            items: self.items.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &Element<T>) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}
