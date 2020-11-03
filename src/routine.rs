//! Create a routine.  
//! ルーチン を作成します。  
use crate::Routine;
use crate::{Control, RoutineBuilder};

impl<T> Default for RoutineBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        RoutineBuilder {
            controls: Vec::new(),
        }
    }
}

impl<T> RoutineBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create operands.  
    //! 項 を作成します。  
    pub fn build(&self) -> Routine<T>
    where
        T: std::clone::Clone,
    {
        Routine {
            controls: self.controls.clone(),
        }
    }

    /// Push the condition.  
    /// 条件を追加します。  
    pub fn push<'a>(&'a mut self, co: &Control<T>) -> &'a mut Self {
        self.controls.push(co.clone());
        self
    }

    /// Push the controls.  
    /// 条件リストを追加します。  
    pub fn extend<'a>(&'a mut self, controls: &Vec<Control<T>>) -> &'a mut Self {
        self.controls.extend(controls.clone());
        self
    }
}

impl<T> Default for Routine<T> {
    fn default() -> Self {
        Routine {
            controls: Vec::new(),
        }
    }
}

impl<T> Routine<T> {
    pub fn get_control(&self, index: usize) -> Option<&Control<T>> {
        if index < self.controls.len() {
            Some(&self.controls[index])
        } else {
            None
        }
    }
}
