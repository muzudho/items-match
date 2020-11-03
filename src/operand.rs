//! Create operands.  
//! 項 を作成します。  
use crate::Operand;
use crate::{OperandsBuilder, OperandsVal};

impl<T> Default for OperandsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        OperandsBuilder {
            operands: Vec::new(),
        }
    }
}

impl<T> OperandsBuilder<T>
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
            operands: self.operands.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &Operand<T>) -> &'a mut Self {
        self.operands.push(item.clone());
        self
    }
}

impl<T> Default for OperandsVal<T> {
    fn default() -> Self {
        OperandsVal {
            operands: Vec::new(),
        }
    }
}
