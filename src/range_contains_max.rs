//! Specify by range. Includes maximum value.  
//! 範囲で指定。最大値を含みます。  
use crate::RangeContainsMax;

impl<T> Default for RangeContainsMax<T> {
    fn default() -> Self {
        RangeContainsMax {
            min: None,
            max: None,
        }
    }
}
