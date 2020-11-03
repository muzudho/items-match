//! Create operands.  
//! 項 を作成します。  
use crate::Condition;
use crate::{OrOperandsBuilder, OrOperandsVal};

impl<T> Default for OrOperandsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        OrOperandsBuilder {
            conditions: Vec::new(),
        }
    }
}

impl<T> OrOperandsBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create operands.  
    //! 項 を作成します。  
    pub fn build(&self) -> OrOperandsVal<T>
    where
        T: std::clone::Clone,
    {
        OrOperandsVal {
            conditions: self.conditions.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, cnd: &Condition<T>) -> &'a mut Self {
        self.conditions.push(cnd.clone());
        self
    }
}

impl<T> Default for OrOperandsVal<T> {
    fn default() -> Self {
        OrOperandsVal {
            conditions: Vec::new(),
        }
    }
}
