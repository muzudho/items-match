//! Create `Any`.  
//! `Any` を作成します。  
use crate::Operand;
use crate::{Any, AnyVal};

impl<T> Default for Any<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        Any { items: Vec::new() }
    }
}

impl<T> Any<T>
where
    T: std::clone::Clone,
{
    //! Create `Any`.  
    //! `Any` を作成します。  
    pub fn build(&self) -> AnyVal<T>
    where
        T: std::clone::Clone,
    {
        AnyVal {
            items: self.items.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &Operand<T>) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}

impl<T> Default for AnyVal<T> {
    fn default() -> Self {
        AnyVal { items: Vec::new() }
    }
}
