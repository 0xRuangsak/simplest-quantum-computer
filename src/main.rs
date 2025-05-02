mod algorithms;
mod gates;
mod state;

use algorithms::{run, Algorithm};
use inquire::Select;

fn main() {
    let options: Vec<(&str, Option<Algorithm>)> = vec![
        ("Bell State", Some(Algorithm::Bell)),
        (
            "Deutsch's Algorithm (constant)",
            Some(Algorithm::DeutschConst),
        ),
        (
            "Deutsch's Algorithm (balanced)",
            Some(Algorithm::DeutschBalanced),
        ),
        ("Superdense Coding", Some(Algorithm::Superdense)),
        ("Parity Check", Some(Algorithm::ParityCheck)),
        ("Grover's Algorithm", Some(Algorithm::Grover)),
        ("Exit", None),
    ];

    let labels: Vec<&str> = options.iter().map(|(label, _)| *label).collect();

    if let Ok(choice) = Select::new("Choose an algorithm to run:", labels).prompt() {
        if let Some((_, Some(algo))) = options.iter().find(|(label, _)| *label == choice) {
            run(algo.clone());
        } else {
            println!("Exiting...");
        }
    }
}
