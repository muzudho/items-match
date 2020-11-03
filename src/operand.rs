//! Create operands.  
//! 項 を作成します。  
use crate::Operand;
use crate::{Operands, OperandsVal};

impl<T> Default for Operands<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        Operands { items: Vec::new() }
    }
}

impl<T> Operands<T>
where
    T: std::clone::Clone,
{
    //! Create operands.  
    //! 項 を作成します。  
    pub fn build(&self) -> OperandsVal<T>
    where
        T: std::clone::Clone,
    {
        OperandsVal {
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

impl<T> Default for OperandsVal<T> {
    fn default() -> Self {
        OperandsVal { items: Vec::new() }
    }
}
