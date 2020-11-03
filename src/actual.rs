//! Create `ActualVal`.  
//! `ActualVal` を作成します。  
use crate::{ActualBuilder, ActualVal};

impl<T> Default for ActualBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ActualBuilder { items: Vec::new() }
    }
}

impl<T> ActualBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `ActualVal`.  
    //! `ActualVal` を作成します。  
    pub fn build(&self) -> ActualVal<T>
    where
        T: std::clone::Clone,
    {
        ActualVal {
            items: self.items.clone(),
        }
    }

    /// Set the number of items to read ahead.  
    /// 先読みする項目数を設定します。  
    pub fn push<'a>(&'a mut self, item: &T) -> &'a mut Self {
        self.items.push(item.clone());
        self
    }
}

impl<T> Default for ActualVal<T> {
    fn default() -> Self {
        ActualVal { items: Vec::new() }
    }
}

impl<T> ActualVal<T> {
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.items.len() {
            Some(&self.items[index])
        } else {
            None
        }
    }

    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}
