use crate::Repeat;

impl<T> Repeat<T> {
    pub fn is_active(&self) -> bool {
        self.min <= self.cursor && self.cursor < self.max
    }
}
