//! Create `ExpectedVal`.  
//! `ExpectedVal` を作成します。  
use crate::{Control, ExpectedBuilder, ExpectedVal};

impl<T> Default for ExpectedBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ExpectedBuilder { items: Vec::new() }
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
            items: self.items.clone(),
        }
    }

    /// Push the comtrol.  
    /// 制御を追加します。  
    pub fn push<'a>(&'a mut self, item: &Control<T>) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}

impl<T> Default for ExpectedVal<T> {
    fn default() -> Self {
        ExpectedVal { items: Vec::new() }
    }
}

impl<T> ExpectedVal<T> {
    pub fn get(&self, index: usize) -> Option<&Control<T>> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }
}
