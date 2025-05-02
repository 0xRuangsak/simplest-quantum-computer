use num_complex::Complex;
use rand::distributions::{Distribution, WeightedIndex};

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

    pub fn measure(&mut self) -> String {
        // Step 1: compute probabilities
        let probabilities: Vec<f64> = self.state.iter().map(|c| c.norm_sqr()).collect();

        // Step 2: sample an index based on probabilities
        let dist = WeightedIndex::new(&probabilities).unwrap();
        let mut rng = rand::thread_rng();
        let collapsed_index = dist.sample(&mut rng);

        // Step 3: collapse state to the measured index
        self.state = self
            .state
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i == collapsed_index {
                    Complex::new(1.0, 0.0)
                } else {
                    Complex::new(0.0, 0.0)
                }
            })
            .collect();

        // Step 4: return the measured bitstring (e.g. "00", "11")
        format!("{:02b}", collapsed_index)
    }

    pub fn from_basis(bits: &str) -> Self {
        let index = usize::from_str_radix(bits, 2).unwrap();
        let mut state = vec![Complex::new(0.0, 0.0); 4];
        state[index] = Complex::new(1.0, 0.0);
        Self { state }
    }
}
