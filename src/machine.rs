use crate::{
    ActualVal, Condition, ConditionsVal, Control, ExpectedVal, MachineBuilder, MachineState,
    MachineVal, MatchingResult, Operator, RangeIncludesMaxVal,
};
use std::fmt;

impl<T> Default for MachineBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        MachineBuilder {
            actual: None,
            expected: None,
        }
    }
}

impl<T> MachineBuilder<T>
where
    T: std::clone::Clone,
{
    //! Create `MachineVal`.  
    //! `MachineVal` を作成します。  
    pub fn build(&self) -> MachineVal<T> {
        MachineVal {
            actual: self.actual.clone().unwrap(),
            expected: self.expected.clone().unwrap(),
        }
    }

    /// Set an actual.  
    pub fn actual<'a>(&'a mut self, ac: &ActualVal<T>) -> &'a mut Self {
        self.actual = Some(ac.clone());
        self
    }
    /// Set an expected.  
    pub fn expected<'a>(&'a mut self, ex: &ExpectedVal<T>) -> &'a mut Self {
        self.expected = Some(ex.clone());
        self
    }
}

impl<T> MachineVal<T>
where
    T: std::cmp::PartialEq + std::cmp::PartialOrd,
{
    /// Compare actual and expected for an exact match.  
    /// 完全一致するかどうか actual と expected を比較します。  
    pub fn exec(&self) -> bool {
        let mut machine_state = MachineState::default();

        for (i, act) in self.actual.get_items().iter().enumerate() {
            machine_state.actual_index = i;
            if machine_state.actual_index + 1 == self.actual.len() {
                machine_state.is_final = true;
            }

            // TODO expected_index カーソルを勧めるのはあとで。
            if let Some(mut co) = self
                .expected
                .get_routine()
                .get_control(machine_state.expected_index)
            {
                match self.matching2(&mut machine_state, act, &mut co) {
                    MatchingResult::Matched => {
                        println!("(trace.66) マッチしたという判断。ループ続行。");
                        machine_state.expected_index += 1;
                        machine_state.matched_length_in_repeat = 0; // reset.
                        machine_state.matched_length_in_seq = 0; // reset.
                    }
                    MatchingResult::NotMatch => {
                        println!("(trace.72) マッチしていないという判断。");
                        return false;
                    }
                    MatchingResult::Ongoing => {
                        println!("(trace.76) マッチ中という判断。ループ続行。");
                    }
                }
            } else {
                // マッチしていないという判断。
                println!("(trace.81) マッチしていないという判断。");
                return false;
            }
        }

        // 失敗していなければ成功という判断。
        println!("(trace.87) 失敗していなければ成功という判断。");
        return true;
    }

    pub fn matching2(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        co: &Control<T>,
    ) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match co {
            Control::Once(co) => match co {
                Operator::Or(cnds) => self.matching4_any(machine_state, act, cnds),
                Operator::One(cnd) => self.matching4_one(machine_state, act, cnd),
            },
            Control::Repeat(rep) => {
                if rep.is_cutoff(machine_state.matched_length_in_repeat) {
                    //  || self.is_final
                    match self.matching3_quantity(machine_state, act, &rep.op) {
                        MatchingResult::NotMatch => {
                            println!("(trace.110) rep={}", rep);
                            return MatchingResult::NotMatch;
                        }
                        MatchingResult::Matched | MatchingResult::Ongoing => {
                            machine_state.matched_length_in_repeat += 1;
                            if rep.is_cutoff(machine_state.matched_length_in_repeat) {
                                //*
                                println!(
                                    "(trace.118) Cutoff. 上限までマッチしたので切上げ。 rep={}",
                                    rep
                                );
                                // */
                                return MatchingResult::Matched;
                            } else if rep.is_success(machine_state.matched_length_in_repeat) {
                                println!("(trace.124) rep={}", rep);
                                return MatchingResult::Matched;
                            } else {
                                println!("(trace.127) fail. rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                    }
                } else {
                    match self.matching3_quantity(machine_state, act, &rep.op) {
                        MatchingResult::NotMatch => {
                            if rep.is_cutoff(machine_state.matched_length_in_repeat) {
                                //*
                                println!(
                                    "(trace.138) Cutoff. 上限までマッチしたので切上げ。 rep={}",
                                    rep
                                );
                                // */
                                return MatchingResult::Matched;
                            } else if rep.is_success(machine_state.matched_length_in_repeat) {
                                //*
                                println!(
                                    "(trace.146) マッチしなくなったところで再判定。 rep={}",
                                    rep
                                );
                                // */
                                return MatchingResult::Matched;
                            } else {
                                println!("(trace.152) fail. rep={}", rep);
                                return MatchingResult::NotMatch;
                            }
                        }
                        MatchingResult::Matched => {
                            machine_state.matched_length_in_repeat += 1;
                            println!("(trace.158) マッチ中なので続行。 rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                        MatchingResult::Ongoing => {
                            machine_state.matched_length_in_repeat += 1;
                            println!("(trace.163) rep={}", rep);
                            return MatchingResult::Ongoing;
                        }
                    }
                }
            }
        }
    }

    pub fn matching3_quantity(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        op: &Operator<T>,
    ) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        match op {
            Operator::Or(cnds) => self.matching4_any(machine_state, act, cnds),
            Operator::One(cnd) => self.matching4_one(machine_state, act, cnd),
        }
    }

    fn matching4_any(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        cnds: &ConditionsVal<T>,
    ) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        for cnd in &cnds.conditions {
            match self.matching4_el(machine_state, act, cnd) {
                MatchingResult::Matched => return MatchingResult::Matched,
                MatchingResult::Ongoing => return MatchingResult::Ongoing,
                MatchingResult::NotMatch => {} // 続行。
            }
        }
        println!("(trace.203) Anyでぜんぶ不一致。");
        return MatchingResult::NotMatch;
    }
    fn matching4_el(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        cnd: &Condition<T>,
    ) -> MatchingResult {
        match cnd {
            Condition::Pin(x) => {
                if *x == *act {
                    println!("(trace.215) matching_any/matched.");
                    MatchingResult::Matched
                } else {
                    MatchingResult::NotMatch
                }
            }
            Condition::RangeIncludesMax(rng) => self.matching5_range_contains_max(act, rng),
            Condition::Seq(vec) => self.matching5_seq(machine_state, act, vec),
        }
    }

    /// One.  
    fn matching4_one(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        cnd: &Condition<T>,
    ) -> MatchingResult
    where
        T: std::cmp::PartialEq + std::cmp::PartialOrd,
    {
        return self.matching4_el(machine_state, act, cnd);
    }

    /// Seq.  
    fn matching5_seq(
        &self,
        machine_state: &mut MachineState,
        act: &T,
        vec: &Vec<T>,
    ) -> MatchingResult {
        let result = if machine_state.matched_length_in_seq < vec.len() + 1 {
            // Ongoing.
            let x = &vec[machine_state.matched_length_in_seq];
            if *x == *act {
                println!("(trace.250) seq/Ongoing machine_state={}", machine_state);
                MatchingResult::Ongoing
            } else {
                println!("(trace.253) seq/NotMatch machine_state={}", machine_state);
                MatchingResult::NotMatch
            }
        } else if machine_state.matched_length_in_seq < vec.len() {
            // Last
            let x = &vec[machine_state.matched_length_in_seq];
            if *x == *act {
                println!(
                    "(trace.260) seq/Last/Matched machine_state={}",
                    machine_state
                );
                MatchingResult::Matched
            } else {
                println!(
                    "(trace.263) seq/Last?NotMatch machine_state={}",
                    machine_state
                );
                MatchingResult::NotMatch
            }
        } else {
            // Last or more.
            panic!("(233) Out of bounds in sequence."); //TODO
        };
        machine_state.matched_length_in_seq += 1;
        return result;
    }

    /// Range.  
    fn matching5_range_contains_max(&self, act: &T, rng: &RangeIncludesMaxVal<T>) -> MatchingResult
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

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            actual_index: 0,
            expected_index: 0,
            is_final: false,
            matched_length_in_repeat: 0,
            matched_length_in_seq: 0,
        }
    }
}
impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("actual_index={} ", self.actual_index));
        buf.push_str(&format!("expected_index={} ", self.expected_index));
        buf.push_str(&format!("is_final={} ", self.is_final));
        buf.push_str(&format!(
            "matched_length_in_repeat={} ",
            self.matched_length_in_repeat
        ));
        buf.push_str(&format!(
            "matched_length_in_seq={} ",
            self.matched_length_in_seq
        ));
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        buf.push_str(&format!("actual_index={} ", self.actual_index));
        buf.push_str(&format!("expected_index={:?} ", self.expected_index));
        buf.push_str(&format!("is_final={} ", self.is_final));
        buf.push_str(&format!(
            "matched_length_in_repeat={} ",
            self.matched_length_in_repeat
        ));
        buf.push_str(&format!(
            "matched_length_in_seq={} ",
            self.matched_length_in_seq
        ));
        write!(f, "{}", buf)
    }
}
