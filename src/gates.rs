use num_complex::Complex;
use std::f64::consts::FRAC_1_SQRT_2; // = 1/√2

pub type Gate = [[Complex<f64>; 4]; 4];

// Pauli-X on qubit 0 (MSB)
pub const X1: Gate = [
    [c(0.0), c(0.0), c(1.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(1.0)],
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(1.0), c(0.0), c(0.0)],
];

// Pauli-X on qubit 1 (LSB)
pub const X2: Gate = [
    [c(0.0), c(1.0), c(0.0), c(0.0)],
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(1.0)],
    [c(0.0), c(0.0), c(1.0), c(0.0)],
];

// Hadamard on qubit 0
pub const H1: Gate = [
    [c(FRAC_1_SQRT_2), c(0.0), c(FRAC_1_SQRT_2), c(0.0)],
    [c(0.0), c(FRAC_1_SQRT_2), c(0.0), c(FRAC_1_SQRT_2)],
    [c(FRAC_1_SQRT_2), c(0.0), c(-FRAC_1_SQRT_2), c(0.0)],
    [c(0.0), c(FRAC_1_SQRT_2), c(0.0), c(-FRAC_1_SQRT_2)],
];

// Hadamard on qubit 1
pub const H2: Gate = [
    [c(FRAC_1_SQRT_2), c(FRAC_1_SQRT_2), c(0.0), c(0.0)],
    [c(FRAC_1_SQRT_2), c(-FRAC_1_SQRT_2), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(FRAC_1_SQRT_2), c(FRAC_1_SQRT_2)],
    [c(0.0), c(0.0), c(FRAC_1_SQRT_2), c(-FRAC_1_SQRT_2)],
];

// CNOT: control qubit 0 → target qubit 1
pub const CNOT12: Gate = [
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(1.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(1.0)],
    [c(0.0), c(0.0), c(1.0), c(0.0)],
];

// CNOT: control qubit 1 → target qubit 0
pub const CNOT21: Gate = [
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(1.0)],
    [c(0.0), c(0.0), c(1.0), c(0.0)],
    [c(0.0), c(1.0), c(0.0), c(0.0)],
];

pub const DEUTSCH_CONST: Gate = [
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(1.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(1.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(1.0)],
];

pub const Z1: Gate = [
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(-1.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(1.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(-1.0)],
];

pub const Z2: Gate = [
    [c(1.0), c(0.0), c(0.0), c(0.0)],
    [c(0.0), c(1.0), c(0.0), c(0.0)],
    [c(0.0), c(0.0), c(-1.0), c(0.0)],
    [c(0.0), c(0.0), c(0.0), c(-1.0)],
];

// Convenience macro for complex constants
const fn c(x: f64) -> Complex<f64> {
    Complex { re: x, im: 0.0 }
}
