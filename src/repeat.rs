use crate::Repeat;

impl<T> Repeat<T> {
    pub fn is_final(&self) -> bool {
        self.cursor + 1 == self.max
    }
    pub fn is_success(&self) -> bool {
        self.min <= self.cursor && self.cursor < self.max
    }
}
