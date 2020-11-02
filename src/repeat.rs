use crate::Repeat;
use std::fmt;

impl<T> Repeat<T> {
    pub fn is_cutoff(&self, matched_length_in_repeat: usize) -> bool {
        self.max_not_included - 1 <= matched_length_in_repeat
    }
    pub fn is_success(&self, matched_length_in_repeat: usize) -> bool {
        self.min <= matched_length_in_repeat && matched_length_in_repeat < self.max_not_included
    }
}
impl<T> fmt::Display for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max_not_included={} ", self.max_not_included));
        write!(f, "{}", buf)
    }
}
