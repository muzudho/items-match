use crate::{ActualItems, ExpectedItem, ExpectedItems, Machine};

impl Default for Machine {
    fn default() -> Self {
        Machine { index: 0 }
    }
}

impl Machine {
    pub fn matching<T>(
        &mut self,
        actual_items: &ActualItems<T>,
        expected_items: &ExpectedItems<T>,
    ) -> bool
    where
        T: std::cmp::PartialEq,
    {
        let act = actual_items.get(self.index);
        let exp = expected_items.get(self.index);

        if let Some(act) = act {
            if let Some(exp) = exp {
                match exp {
                    ExpectedItem::Any(any) => {
                        for exp in &any.items {
                            if *exp == *act {
                                return true;
                            }
                        }
                        return false;
                    }
                    ExpectedItem::Exact(exp) => {
                        if *exp == *act {
                            true
                        } else {
                            false
                        }
                    }
                }
            } else {
                return false;
            }
        } else {
            if let Some(_exp) = exp {
                return false;
            } else {
                // None vs None はマッチしていないという判断。
                return false;
                // return act == exp;
            }
        }
    }
}
