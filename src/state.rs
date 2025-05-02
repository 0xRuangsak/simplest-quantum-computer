use num_complex::Complex;

pub struct QuantumState {
    pub state: Vec<Complex<f64>>,
}

impl QuantumState {
    pub fn new() -> Self {
        let state = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];

        Self { state }
    }

    pub fn is_normalized(&self) -> bool {
        let norm: f64 = self.state.iter().map(|c| c.norm_sqr()).sum();

        (norm - 1.0).abs() < 1e-10
    }
}
