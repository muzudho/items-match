use crate::ActualVal;
use crate::Expected;
use crate::{Machine, MachineBuilder};

impl<T> Default for MachineBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        MachineBuilder {
            actual: None,
            expected: None,
        }
    }
}

impl<T> MachineBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `Machine`.  
    //! `Machine` を作成します。  
    pub fn build(&self) -> Machine<T> {
        Machine {
            actual: self.actual.clone().unwrap(),
            expected: self.expected.clone().unwrap(),
        }
    }

    pub fn set_actual<'a>(&'a mut self, item: &ActualVal<T>) -> &'a mut Self {
        self.actual = Some(item.clone());
        self
    }
    pub fn set_expected<'a>(&'a mut self, item: &Expected<T>) -> &'a mut Self {
        self.expected = Some(item.clone());
        self
    }
}
