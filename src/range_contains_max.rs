//! Specify by range. Includes maximum value.  
//! 範囲で指定。最大値を含みます。  
use crate::RangeIncludesMax;

impl<T> Default for RangeIncludesMax<T> {
    fn default() -> Self {
        RangeIncludesMax {
            min: None,
            max: None,
        }
    }
}
