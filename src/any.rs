use crate::Any;

impl<T> Default for Any<T> {
    fn default() -> Self {
        Any { items: Vec::new() }
    }
}
