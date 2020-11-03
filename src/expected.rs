//! Create `ExpectedVal`.  
//! `ExpectedVal` を作成します。  
use crate::{ExpectedBuilder, ExpectedVal, Routine};

impl<T> Default for ExpectedBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ExpectedBuilder {
            routine: Routine::default(),
        }
    }
}

impl<T> ExpectedBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `ExpectedVal`.  
    //! `ExpectedVal` を作成します。  
    pub fn build(&self) -> ExpectedVal<T>
    where
        T: std::clone::Clone,
    {
        ExpectedVal {
            routine: self.routine.clone(),
        }
    }

    /// Set a routine.  
    /// ルーチンを設定します。  
    pub fn routine<'a>(&'a mut self, ro: &Routine<T>) -> &'a mut Self {
        self.routine = ro.clone();
        self
    }
}

impl<T> Default for ExpectedVal<T> {
    fn default() -> Self {
        ExpectedVal {
            routine: Routine::default(),
        }
    }
}

impl<T> ExpectedVal<T> {
    pub fn get_routine(&self) -> &Routine<T> {
        &self.routine
    }
}
