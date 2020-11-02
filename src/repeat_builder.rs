//! Create `Repeat`.  
//! `Repeat` を作成します。  
use crate::Quantity;
use crate::{Repeat, RepeatBuilder};

impl<T> Default for RepeatBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        RepeatBuilder {
            expected: None,
            min: 0,
            max: 0,
        }
    }
}

impl<T> RepeatBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `Repeat`.  
    //! `Repeat` を作成します。  
    pub fn build(&self) -> Repeat<T>
    where
        T: std::clone::Clone,
    {
        Repeat {
            expected: self.expected.clone().unwrap(),
            min: self.min,
            max: self.max,
            matched_length: 0,
        }
    }

    /// 最低何回繰り返すか。  
    pub fn set_expected<'a>(&'a mut self, expected: &Quantity<T>) -> &'a mut Self {
        self.expected = Some(Box::new(expected.clone()));
        self
    }
    /// 最低何回繰り返すか。  
    pub fn set_min<'a>(&'a mut self, val: usize) -> &'a mut Self {
        self.min = val;
        self
    }

    /// 最大何回繰り返すか。  
    pub fn set_max<'a>(&'a mut self, val: usize) -> &'a mut Self {
        self.max = val;
        self
    }
}
