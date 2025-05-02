mod gates;
mod state;

use gates::{CNOT12, H1};
use state::QuantumState;

fn main() {
    let mut q = QuantumState::new();

    // Step 1: Apply H to qubit 0
    q.apply_gate(&H1);

    // Step 2: Apply CNOT: control=qubit 0, target=qubit 1
    q.apply_gate(&CNOT12);

    println!("Bell state: {:?}", q.state);
    println!("Is normalized? {}", q.is_normalized());
}
