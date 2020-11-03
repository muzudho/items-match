//! Specify by range. Includes maximum value.  
//! 範囲で指定。最大値を含みます。  
//! Create `RangeIncludesMaxVal`.  
//! `RangeIncludesMaxVal` を作成します。  

use crate::{RangeIncludesMax, RangeIncludesMaxVal};

impl<T> Default for RangeIncludesMax<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        RangeIncludesMax {
            min: None,
            max: None,
        }
    }
}

impl<T> RangeIncludesMax<T>
where
    T: std::clone::Clone,
{
    //! Create `RangeIncludesMaxVal`.  
    //! `RangeIncludesMaxVal` を作成します。  
    pub fn build(&self) -> RangeIncludesMaxVal<T>
    where
        T: std::clone::Clone,
    {
        RangeIncludesMaxVal {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }

    /// Set a max.
    pub fn max<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.max = Some(item.clone());
        self
    }
    /// Set a min.
    pub fn min<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.min = Some(item.clone());
        self
    }
}

impl<T> Default for RangeIncludesMaxVal<T> {
    fn default() -> Self {
        RangeIncludesMaxVal {
            min: None,
            max: None,
        }
    }
}
