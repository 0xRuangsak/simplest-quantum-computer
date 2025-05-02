use crate::gates::*;
use crate::state::QuantumState;

pub fn run_bell_state() {
    let mut q = QuantumState::new();
    q.apply_gate(&H1);
    q.apply_gate(&CNOT12);
    let result = q.measure();
    println!("Measured result: {}", result);
}
