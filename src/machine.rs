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

            // TODO expected_index カーソルを勧めるのはあとで。
            if let Some(mut exp) = expected_items.get_mut(self.expected_index) {
                match self.matching2(act, &mut exp) {
                    MatchingResult::Matched => {
                        println!("(trace.30)");
                        self.expected_index += 1;
                        return true;
                    }
                    MatchingResult::NotMatch => {
                        println!("(trace.35)");
                        return false;
                    }
                    MatchingResult::Ongoing => {
                        println!("(trace.38) ループ続行。");
                    }
                }
            } else {
                // マッチしていないという判断。
                println!("(trace.44)");
                return false;
            }
        }

        // 失敗していなければ成功という判断。
        println!("(trace.51)");
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
                        println!("(trace.63) matching2/matched.");
                        return MatchingResult::Matched;
                    }
                }
                println!("(trace.67) Anyで不一致。");
                return MatchingResult::NotMatch;
            }
            Expected::Exact(exp) => {
                if *exp == *act {
                    println!("(trace.72)");
                    MatchingResult::Matched
                } else {
                    println!("(trace.75)");
                    MatchingResult::NotMatch
                }
            }
            Expected::Repeat(rep) => {
                if rep.is_final() {
                    rep.cursor += 1;

                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => return MatchingResult::NotMatch,
                        MatchingResult::Matched | MatchingResult::Ongoing => {
                            if rep.is_success() {
                                println!("(trace.87) rep={}", rep);
                                return MatchingResult::Matched;
                            } else {
                                println!("(trace.90) rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                    }
                } else {
                    rep.cursor += 1;
                    // 再帰的
                    match self.matching2(act, &mut rep.expected) {
                        MatchingResult::NotMatch => return MatchingResult::NotMatch,
                        MatchingResult::Matched => {
                            println!("(trace.101) マッチ中なので続行。 rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                        MatchingResult::Ongoing => {
                            println!("(trace.109) rep={}", rep);
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
