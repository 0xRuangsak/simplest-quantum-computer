# Simplest Quantum Computer Simulator

A minimal 2-qubit quantum computer simulator written in Rust.  
It supports gate operations, entanglement, measurement, and several foundational quantum algorithms through an interactive CLI.

## Features

- 2-qubit complex state simulation
- Gate operations: X, Z, H, CNOT
- Measurement with probabilistic collapse
- Interactive CLI with algorithm selection
- Included algorithms:
  - Bell State
  - Deutsch’s Algorithm (constant and balanced)
  - Superdense Coding
  - Parity Check
  - Grover’s Algorithm (2-qubit version)

## Usage

```bash
cargo run
````

Use arrow keys to select an algorithm from the menu.

## Structure

* `main.rs` – CLI entry point
* `state.rs` – quantum state and measurement logic
* `gates.rs` – quantum gates and oracles
* `algorithms.rs` – algorithm implementations

## Dependencies

* `num-complex` – complex number support
* `rand` – probabilistic measurement
* `inquire` – interactive CLI prompt

## License

MIT License

