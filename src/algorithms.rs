use crate::gates::*;
use crate::state::QuantumState;
use num_complex::Complex;

#[derive(Clone)]
/// Entry point for running a named algorithm
pub enum Algorithm {
    Bell,
    DeutschConst,
    DeutschBalanced,
}

pub fn run(algorithm: Algorithm) {
    match algorithm {
        Algorithm::Bell => run_bell_state(),
        Algorithm::DeutschConst => run_deutsch_const(),
        Algorithm::DeutschBalanced => run_deutsch_balanced(),
    }
}

/// Bell state (entanglement example)
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

/// Deutsch’s Algorithm: constant oracle
pub fn run_deutsch_const() {
    run_deutsch(DEUTSCH_CONST, "f(x) = 0 (constant)");
}

/// Deutsch’s Algorithm: balanced oracle
pub fn run_deutsch_balanced() {
    run_deutsch(CNOT12, "f(x) = x (balanced)");
}

/// Shared Deutsch circuit
fn run_deutsch(oracle: [[Complex<f64>; 4]; 4], label: &str) {
    println!("\n[Deutsch's Algorithm] Oracle: {}", label);

    println!("\n[Step 1] Initialize state to |01⟩");
    let mut q = QuantumState::from_basis("01");
    print_state(&q);

    println!("\n[Step 2] Apply H to both qubits");
    q.apply_gate(&H1);
    q.apply_gate(&H2);
    print_state(&q);

    println!("\n[Step 3] Apply oracle");
    q.apply_gate(&oracle);
    print_state(&q);

    println!("\n[Step 4] Apply H to qubit 0");
    q.apply_gate(&H1);
    print_state(&q);

    println!("\n[Step 5] Measure qubit 0");
    let collapsed = q.measure();
    println!("→ Measured: |{}⟩", collapsed);
    match &collapsed[..1] {
        "0" => println!("→ f is constant"),
        "1" => println!("→ f is balanced"),
        _ => println!("→ Invalid result"),
    }
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
