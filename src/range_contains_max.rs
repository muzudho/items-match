use crate::RangeContainsMax;

impl<T> Default for RangeContainsMax<T> {
    fn default() -> Self {
        RangeContainsMax {
            min: None,
            max: None,
        }
    }
}
