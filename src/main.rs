mod state;

use state::QuantumState;

fn main() {
    let q = QuantumState::new();
    println!("Initial state: {:?}", q.state);
    println!("Is normalized? {}", q.is_normalized());
}
