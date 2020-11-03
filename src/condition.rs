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

    /// Push the condition.  
    /// 条件を追加します。  
    pub fn push<'a>(&'a mut self, cnd: &Condition<T>) -> &'a mut Self {
        self.conditions.push(cnd.clone());
        self
    }

    /// Push the conditions.  
    /// 条件リストを追加します。  
    pub fn extend<'a>(&'a mut self, cnds: &ConditionsVal<T>) -> &'a mut Self {
        self.conditions.extend(cnds.conditions.clone());
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
