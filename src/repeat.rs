//! Create `Repeat`.  
//! `Repeat` を作成します。  

use crate::OrOperator;
use crate::{Repeat, RepeatVal};
use std::fmt;

impl<T> Default for Repeat<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        Repeat {
            quantity: None,
            min: 0,
            max_not_included: 0,
        }
    }
}

impl<T> Repeat<T>
where
    T: std::clone::Clone,
{
    //! Create `Repeat`.  
    //! `Repeat` を作成します。  
    pub fn build(&self) -> RepeatVal<T>
    where
        T: std::clone::Clone,
    {
        RepeatVal {
            quantity: self.quantity.clone().unwrap(),
            min: self.min,
            max_not_included: self.max_not_included,
        }
    }

    /// Set a quantity.  
    /// 最低何回繰り返すか。  
    pub fn quantity<'a>(&'a mut self, quantity: &OrOperator<T>) -> &'a mut Self {
        self.quantity = Some(Box::new(quantity.clone()));
        self
    }
    /// Set a min.  
    /// 最低何回繰り返すか。  
    pub fn min<'a>(&'a mut self, val: usize) -> &'a mut Self {
        self.min = val;
        self
    }

    /// Set max not included.  
    /// 最大何回繰り返すか。最大値は含まない。  
    pub fn max_not_included<'a>(&'a mut self, val: usize) -> &'a mut Self {
        self.max_not_included = val;
        self
    }
}

impl<T> RepeatVal<T> {
    pub fn is_cutoff(&self, matched_length_in_repeat: usize) -> bool {
        self.max_not_included - 1 <= matched_length_in_repeat
    }
    pub fn is_success(&self, matched_length_in_repeat: usize) -> bool {
        self.min <= matched_length_in_repeat && matched_length_in_repeat < self.max_not_included
    }
}
impl<T> fmt::Display for RepeatVal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for RepeatVal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        write!(f, "{}", buf)
    }
}
