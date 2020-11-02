use crate::MatchingResult;
use crate::{ActualItems, Expected, ExpectedItems, Machine};
use std::fmt;

impl Default for Machine {
    fn default() -> Self {
        Machine {
            actual_index: 0,
            expected_index: 0,
        }
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
        for (i, act) in actual_items.get_items().iter().enumerate() {
            self.actual_index = i;
            let exp = expected_items.get_mut(self.expected_index); // TODO カーソルを勧めるのはあとで。

            if let Some(mut exp) = exp {
                match self.matching2(act, &mut exp) {
                    MatchingResult::Matched => {
                        self.expected_index += 1;
                        return true;
                    }
                    MatchingResult::NotMatch => return false,
                    MatchingResult::Ongoing => return true,
                }
            } else {
                // マッチしていないという判断。
                return false;
            }
        }

        // 失敗していなければ成功という判断。
        return true;
    }

    pub fn matching2<T>(&mut self, act: &T, exp: &mut Expected<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq,
    {
        match exp {
            Expected::Any(any) => {
                for exp in &any.items {
                    if *exp == *act {
                        return MatchingResult::Matched;
                    }
                }
                return MatchingResult::NotMatch;
            }
            Expected::Exact(exp) => {
                if *exp == *act {
                    MatchingResult::Matched
                } else {
                    MatchingResult::NotMatch
                }
            }
            Expected::Repeat(rep) => {
                if rep.is_final() {
                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => return MatchingResult::NotMatch,
                        MatchingResult::Matched | MatchingResult::Ongoing => {
                            if rep.is_success() {
                                rep.cursor += 1;
                                return MatchingResult::Matched;
                            } else {
                                return MatchingResult::NotMatch;
                            }
                        }
                    }
                } else {
                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => return MatchingResult::NotMatch,
                        MatchingResult::Matched => {
                            if rep.is_success() {
                                return MatchingResult::Matched;
                            } else {
                                return MatchingResult::NotMatch;
                            }
                        }
                        MatchingResult::Ongoing => {
                            return MatchingResult::Ongoing;
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("actual_index={} ", self.actual_index));
        buf.push_str(&format!("expected_index={} ", self.expected_index));
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("actual_index={} ", self.actual_index));
        buf.push_str(&format!("expected_index={:?} ", self.expected_index));
        write!(f, "{}", buf)
    }
}
