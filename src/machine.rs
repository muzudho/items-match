use crate::{ActualItems, Expected, ExpectedItems, Machine};

impl Default for Machine {
    fn default() -> Self {
        Machine { index: 0 }
    }
}

impl Machine {
    pub fn matching<T>(
        &mut self,
        actual_items: &ActualItems<T>,
        expected_items: &mut ExpectedItems<T>,
    ) -> bool
    where
        T: std::cmp::PartialEq,
    {
        let act = actual_items.get(self.index);
        let exp = expected_items.get_mut(self.index);

        if let Some(act) = act {
            if let Some(mut exp) = exp {
                return self.matching2(act, &mut exp);
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

    pub fn matching2<T>(&mut self, act: &T, exp: &mut Expected<T>) -> bool
    where
        T: std::cmp::PartialEq,
    {
        match exp {
            Expected::Any(any) => {
                for exp in &any.items {
                    if *exp == *act {
                        return true;
                    }
                }
                return false;
            }
            Expected::Exact(exp) => {
                if *exp == *act {
                    true
                } else {
                    false
                }
            }
            Expected::Repeat(rep) => {
                if rep.is_allow() {
                    // 再帰的
                    let ret = self.matching2(act, &mut rep.expected);
                    rep.cursor += 1;
                    return ret;
                } else {
                    if rep.is_success() {
                        return true;
                    } else {
                        // マッチしていないという判断。
                        return false;
                    }
                }
            }
        }
    }
}
