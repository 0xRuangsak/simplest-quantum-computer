mod gates;
mod state;

use gates::H1;
use state::QuantumState;

fn main() {
    let mut q = QuantumState::new();
    q.apply_gate(&H1);
    println!("Initial state: {:?}", q.state);
    println!("Is normalized? {}", q.is_normalized());
}
