use crate::gates::*;
use crate::state::QuantumState;

#[derive(Clone)]

/// Entry point for running a named algorithm
pub enum Algorithm {
    Bell,
    // Add more: Deutsch, Teleport, etc.
}

pub fn run(algorithm: Algorithm) {
    match algorithm {
        Algorithm::Bell => run_bell_state(),
    }
}

/// Bell state
pub fn run_bell_state() {
    println!("\n[Step 1] Initialize quantum state to |00⟩");
    let mut q = QuantumState::new();
    print_state(&q);

    println!("\n[Step 2] Apply Hadamard gate (H) to qubit 0");
    q.apply_gate(&H1);
    print_state(&q);

    println!("\n[Step 3] Apply CNOT gate (control: qubit 0 → target: qubit 1)");
    q.apply_gate(&CNOT12);
    print_state(&q);

    println!("\n[Step 4] Measure the system");
    let result = q.measure();
    print_state(&q);
    println!("→ Measurement result: |{}⟩", result);
}

/// Utility: pretty-print the state vector
fn print_state(q: &QuantumState) {
    println!("Current state vector:");
    for (i, amplitude) in q.state.iter().enumerate() {
        let basis = format!("{:02b}", i);
        if amplitude.norm_sqr() > 1e-6 {
            println!("  |{}⟩ → {:.4} + {:.4}i", basis, amplitude.re, amplitude.im);
        }
    }
}
