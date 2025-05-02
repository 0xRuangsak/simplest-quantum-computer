mod algorithms;
mod gates;
mod state;

use algorithms::run_bell_state;
use inquire::Select;

fn main() {
    let options = vec!["Bell State", "Exit"];

    let selection = Select::new("Choose an algorithm to run:", options).prompt();

    match selection {
        Ok("Bell State") => run_bell_state(),
        Ok("Exit") | _ => {
            println!("Exiting...");
        }
    }
}
