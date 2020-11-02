use crate::Repeat;

impl<T> Repeat<T> {
    pub fn is_allow(&self) -> bool {
        self.cursor < self.max
    }
    pub fn is_success(&self) -> bool {
        self.min <= self.cursor && self.cursor < self.max
    }
}
