use crate::Any;
use crate::MatchingResult;
use crate::RangeContainsMax;
use crate::{ActualItems, Controls, Expected, ExpectedItems, Machine, Quantity};
use std::fmt;

impl Default for Machine {
    fn default() -> Self {
        Machine {
            actual_index: 0,
            expected_index: 0,
            is_final: false,
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
            if self.actual_index + 1 == actual_items.len() {
                self.is_final = true;
            }

            // TODO expected_index カーソルを勧めるのはあとで。
            if let Some(mut exp) = expected_items.get_mut(self.expected_index) {
                match self.matching2(act, &mut exp) {
                    MatchingResult::Matched => {
                        println!("(trace.30) マッチしたという判断。");
                        self.expected_index += 1;
                        return true;
                    }
                    MatchingResult::NotMatch => {
                        println!("(trace.35) マッチしていないという判断。");
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
        println!("(trace.51) 失敗していなければ成功という判断。");
        return true;
    }

    pub fn matching2<T>(&mut self, act: &T, exp: &mut Controls<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match exp {
            Controls::Once(exp) => match exp {
                Quantity::Any(any) => self.matching4_any(act, any),
                Quantity::One(exp) => self.matching4_one(act, exp),
            },
            Controls::Repeat(rep) => {
                if rep.is_cutoff() {
                    //  || self.is_final
                    match self.matching3_quantity(act, &mut rep.quantity) {
                        MatchingResult::NotMatch => {
                            println!("(trace.85) rep={}", rep);
                            return MatchingResult::NotMatch;
                        }
                        MatchingResult::Matched | MatchingResult::Ongoing => {
                            rep.matched_length += 1;
                            if rep.is_cutoff() {
                                println!(
                                    "(trace.93) Cutoff. 上限までマッチしたので切上げ。 rep={}",
                                    rep
                                );
                                return MatchingResult::Matched;
                            } else if rep.is_success() {
                                println!("(trace.87) rep={}", rep);
                                return MatchingResult::Matched;
                            } else {
                                println!("(trace.90) fail. rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                    }
                } else {
                    match self.matching3_quantity(act, &mut rep.quantity) {
                        MatchingResult::NotMatch => {
                            if rep.is_cutoff() {
                                println!(
                                    "(trace.93) Cutoff. 上限までマッチしたので切上げ。 rep={}",
                                    rep
                                );
                                return MatchingResult::Matched;
                            } else if rep.is_success() {
                                //*
                                println!(
                                    "(trace.104) マッチしなくなったところで再判定。 rep={}",
                                    rep
                                );
                                // */
                                return MatchingResult::Matched;
                            } else {
                                println!("(trace.107) fail. rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                        MatchingResult::Matched => {
                            rep.matched_length += 1;
                            println!("(trace.112) マッチ中なので続行。 rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                        MatchingResult::Ongoing => {
                            rep.matched_length += 1;
                            println!("(trace.115) rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                    }
                }
            }
        }
    }

    pub fn matching3_quantity<T>(&mut self, act: &T, exp: &mut Quantity<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match exp {
            Quantity::Any(any) => self.matching4_any(act, any),
            Quantity::One(exp) => self.matching4_one(act, exp),
        }
    }

    fn matching4_any<T>(&mut self, act: &T, any: &Any<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        for exp in &any.items {
            match exp {
                Expected::Exact(exa) => {
                    if *exa == *act {
                        println!("(trace.138) matching_any/matched.");
                        return MatchingResult::Matched;
                    }
                }
                Expected::RangeContainsMax(rng) => {
                    match self.matching5_range_contains_max(act, rng) {
                        MatchingResult::Matched => {
                            println!("(trace.138) matching_any/rng/matched.");
                            return MatchingResult::Matched;
                        }
                        MatchingResult::Ongoing => {
                            println!("(trace.138) matching_any/rng/ongoing.");
                            return MatchingResult::Ongoing;
                        }
                        MatchingResult::NotMatch => {
                            // 続行。
                            println!("(trace.138) matching_any/rng/notmatch.");
                        }
                    }
                }
            }
        }
        println!("(trace.67) Anyでぜんぶ不一致。");
        return MatchingResult::NotMatch;
    }
    fn matching4_one<T>(&mut self, act: &T, exp: &Expected<T>) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match exp {
            Expected::Exact(exa) => {
                if *exa == *act {
                    println!("(trace.72)");
                    MatchingResult::Matched
                } else {
                    println!("(trace.75)");
                    MatchingResult::NotMatch
                }
            }
            Expected::RangeContainsMax(rng) => {
                return self.matching5_range_contains_max(act, rng);
            }
        }
    }
    fn matching5_range_contains_max<T>(
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
