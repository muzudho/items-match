//! Create operands.  
//! 項 を作成します。  
use crate::Condition;
use crate::{ConditionsBuilder, ConditionsVal};

impl<T> Default for ConditionsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ConditionsBuilder {
            conditions: Vec::new(),
        }
    }
}

impl<T> ConditionsBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create operands.  
    //! 項 を作成します。  
    pub fn build(&self) -> ConditionsVal<T>
    where
        T: std::clone::Clone,
    {
        ConditionsVal {
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

impl<T> Default for ConditionsVal<T> {
    fn default() -> Self {
        ConditionsVal {
            conditions: Vec::new(),
        }
    }
}
