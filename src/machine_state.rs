use crate::MachineState;

impl Default for MachineState {
    fn default() -> Self {
        MachineState {
            actual_index: 0,
            expected_index: 0,
            is_final: false,
            matched_length_in_repeat: 0,
        }
    }
}
