use crate::Actual;
use crate::Expected;
use crate::{Machine, MachineBuilder};

impl<T> Default for MachineBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        MachineBuilder {
            actual_items: None,
            expected_items: None,
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
            actual_items: self.actual_items.clone().unwrap(),
            expected_items: self.expected_items.clone().unwrap(),
        }
    }

    pub fn set_actual_items<'a>(&'a mut self, item: &Actual<T>) -> &'a mut Self {
        self.actual_items = Some(item.clone());
        self
    }
    pub fn set_expected_items<'a>(&'a mut self, item: &Expected<T>) -> &'a mut Self {
        self.expected_items = Some(item.clone());
        self
    }
}
