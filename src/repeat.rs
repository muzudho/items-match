use crate::Repeat;
use std::fmt;

impl<T> Repeat<T> {
    pub fn is_cutoff(&self) -> bool {
        self.max_not_included - 1 <= self.matched_length
    }
    pub fn is_success(&self) -> bool {
        self.min <= self.matched_length && self.matched_length < self.max_not_included
    }
}
impl<T> fmt::Display for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        buf.push_str(&format!("matched_length={} ", self.matched_length));
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        buf.push_str(&format!("matched_length={} ", self.matched_length));
        write!(f, "{}", buf)
    }
}
