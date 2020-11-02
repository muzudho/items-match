//! Create `RangeIncludesMax`.  
//! `RangeIncludesMax` を作成します。  
use crate::{RangeContainsMaxBuilder, RangeIncludesMax};

impl<T> Default for RangeContainsMaxBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        RangeContainsMaxBuilder {
            min: None,
            max: None,
        }
    }
}

impl<T> RangeContainsMaxBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `RangeIncludesMax`.  
    //! `RangeIncludesMax` を作成します。  
    pub fn build(&self) -> RangeIncludesMax<T>
    where
        T: std::clone::Clone,
    {
        RangeIncludesMax {
            min: self.min.clone(),
            max: self.max.clone(),
        }
    }

    pub fn set_max<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.max = Some(item.clone());
        self
    }
    pub fn set_min<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.min = Some(item.clone());
        self
    }
}
