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

    pub fn apply_gate(&mut self, gate: &[[Complex<f64>; 4]; 4]) {
        let mut new_state = vec![Complex::new(0.0, 0.0); 4];

        for i in 0..4 {
            for j in 0..4 {
                new_state[i] += gate[i][j] * self.state[j];
            }
        }

        self.state = new_state;
    }
}
