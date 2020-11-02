use crate::MatchingResult;
use crate::{ActualItems, Expected, ExpectedItems, Machine};

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
        let act = actual_items.get(self.actual_index);
        self.actual_index += 1;
        let exp = expected_items.get_mut(self.expected_index); // TODO カーソルを勧めるのはあとで。

        if let Some(act) = act {
            if let Some(mut exp) = exp {
                match self.matching2(act, &mut exp) {
                    MatchingResult::Matched => return true,
                    MatchingResult::NotMatch => return false,
                    MatchingResult::Ongoing => panic!("ここでOngoingはおかしい。"),
                }
            } else {
                // マッチしていないという判断。
                return false;
            }
        } else {
            if let Some(_exp) = exp {
                // マッチしていないという判断。
                return false;
            } else {
                // None vs None はマッチしていないという判断。
                return false;
                // return act == exp;
            }
        }
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
