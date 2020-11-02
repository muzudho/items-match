use crate::Repeat;
use std::fmt;

impl<T> Repeat<T> {
    pub fn is_final(&self) -> bool {
        self.max - 1 <= self.cursor
    }
    pub fn is_success(&self) -> bool {
        self.min <= self.cursor && self.cursor < self.max
    }
}
impl<T> fmt::Display for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max={} ", self.max));
        buf.push_str(&format!("cursor={} ", self.cursor));
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for Repeat<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("min={} ", self.min));
        buf.push_str(&format!("max={} ", self.max));
        buf.push_str(&format!("cursor={} ", self.cursor));
        write!(f, "{}", buf)
    }
}
