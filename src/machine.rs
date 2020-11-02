use crate::Any;
use crate::MatchingResult;
use crate::RangeContainsMax;
use crate::{ActualItems, Controls, Expected, ExpectedItems, Machine};
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
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        for (i, act) in actual_items.get_items().iter().enumerate() {
            self.actual_index = i;

            // TODO expected_index カーソルを勧めるのはあとで。
            if let Some(mut exp) = expected_items.get_mut(self.expected_index) {
                match self.matching2(act, &mut exp) {
                    MatchingResult::Matched => {
                        // println!("(trace.30) マッチしたという判断。");
                        self.expected_index += 1;
                        return true;
                    }
                    MatchingResult::NotMatch => {
                        // println!("(trace.35) マッチしていないという判断。");
                        return false;
                    }
                    MatchingResult::Ongoing => {
                        // println!("(trace.38) ループ続行。");
                    }
                }
            } else {
                // マッチしていないという判断。
                // println!("(trace.44)");
                return false;
            }
        }

        // 失敗していなければ成功という判断。
        // println!("(trace.51)");
        return true;
    }

    pub fn matching2<T>(&mut self, act: &T, exp: &mut Controls<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match exp {
            /*
            Controls::Any(any) => self.matching_any(act, any),
            Controls::RangeContainsMax(rng) => self.matching_range_contains_max(act, rng),
            Controls::Exact(exa) => self.matching_exact(act, exa),
            */
            Controls::Once(exp) => match exp {
                Expected::Any(any) => self.matching_any(act, any),
                Expected::Exact(exa) => self.matching_exact(act, exa),
                Expected::RangeContainsMax(rng) => self.matching_range_contains_max(act, rng),
            },
            Controls::Repeat(rep) => {
                if rep.is_final() {
                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => {
                            // println!("(trace.85) rep={}", rep);
                            return MatchingResult::NotMatch;
                        }
                        MatchingResult::Matched | MatchingResult::Ongoing => {
                            rep.matched_length += 1;
                            if rep.is_success() {
                                // println!("(trace.87) rep={}", rep);
                                return MatchingResult::Matched;
                            } else {
                                // println!("(trace.90) rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                    }
                } else {
                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => {
                            if rep.is_success() {
                                /*
                                // println!(
                                    "(trace.104) マッチしなくなったところで再判定。 rep={}",
                                    rep
                                );
                                */
                                return MatchingResult::Matched;
                            } else {
                                // println!("(trace.107) rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                        MatchingResult::Matched => {
                            rep.matched_length += 1;
                            // println!("(trace.112) マッチ中なので続行。 rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                        MatchingResult::Ongoing => {
                            rep.matched_length += 1;
                            // println!("(trace.115) rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                    }
                }
            }
        }
    }

    fn matching_any<T>(&mut self, act: &T, any: &Any<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        for exp in &any.items {
            if *exp == *act {
                // println!("(trace.63) matching2/matched.");
                return MatchingResult::Matched;
            }
        }
        // println!("(trace.67) Anyで不一致。");
        return MatchingResult::NotMatch;
    }
    fn matching_range_contains_max<T>(
        &mut self,
        act: &T,
        rng: &RangeContainsMax<T>,
    ) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        if let Some(min) = &rng.min {
            if *act < *min {
                return MatchingResult::NotMatch;
            }
        }
        if let Some(max) = &rng.max {
            if *max < *act {
                return MatchingResult::NotMatch;
            }
        }
        return MatchingResult::Matched;
    }
    fn matching_exact<T>(&mut self, act: &T, exp: &T) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        if *exp == *act {
            // println!("(trace.72)");
            MatchingResult::Matched
        } else {
            // println!("(trace.75)");
            MatchingResult::NotMatch
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
