use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rnn {
    weights: Vec<Vec<f32>>,
    offsets: Vec<f32>,
    pub state: Vec<f32>,
}

impl Rnn {
    pub fn new(size: usize) -> Self {
        let state: Vec<f32> = vec![0.0; size];
        let offsets: Vec<f32> = vec![0.0; size];
        let mut weights: Vec<Vec<f32>> = Vec::with_capacity(size);
        for _i in 0..size {
            weights.push(vec![0.0; size]);
        }

        Self {
            state,
            offsets,
            weights,
        }
    }

    pub fn mutate_weights(&mut self, scale: f32) {
        let normal = Normal::new(0.0, scale).unwrap();
        let rng = &mut thread_rng();

        for w_arr in self.weights.iter_mut() {
            for w in w_arr.iter_mut() {
                *w += normal.sample(rng) * scale;
            }
        }

        for o in self.offsets.iter_mut() {
            *o += normal.sample(rng) * scale;
        }
    }

    pub fn step(&mut self) {
        assert_eq!(self.state.len(), self.offsets.len());
        assert_eq!(self.state.len(), self.weights.len());

        let prev_state = self.state.clone();
        for (i, weight_array) in self.weights.iter().enumerate() {
            assert_eq!(self.weights.len(), weight_array.len());
            let mut input_gain = -self.offsets[i];
            input_gain += dot_arrays(&prev_state, weight_array);
            self.state[i] = relu_activation_function(input_gain);
        }
    }

    pub fn set_input(&mut self, values: &[f32]) {
        assert!(values.len() < self.state.len());
        for i in 0..values.len() {
            self.state[i] = values[i];
        }
    }

    pub fn read_output(&mut self, num_values: usize) -> Vec<f32> {
        let end = self.state.len();
        assert!(num_values < end);

        return self.state[end - num_values..end].to_vec();
    }
}

fn dot_arrays(a: &[f32], b: &[f32]) -> f32 {
    return a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
}
fn relu_activation_function(val: f32) -> f32 {
    return f32::max(val, 0.0);
}
