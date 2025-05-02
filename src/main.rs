mod algorithms;
mod gates;
mod state;

use algorithms::run_bell_state;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <algorithm>");
        eprintln!("Available: bell");
        return;
    }

    match args[1].as_str() {
        "bell" => run_bell_state(),
        _ => eprintln!("Unknown algorithm: {}", args[1]),
    }
}
