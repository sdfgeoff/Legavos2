use neural::Rnn;
use serde::{Deserialize, Serialize};

use super::probability::{gaussian_approximations_from_array, ProbabilityDistributionFunction};

#[derive(Serialize, Deserialize, Debug)]
pub struct Controller {
    pub brain: Rnn,
}

impl Controller {
    pub fn step(&mut self, state: &[f32], predicted_state: &[f32], num_actions: usize) -> Vec<f32> {
        let mut all_inputs: Vec<f32> = vec![];
        all_inputs.extend_from_slice(state);
        all_inputs.extend_from_slice(predicted_state);

        self.brain.set_input(&all_inputs);
        self.brain.step();

        self.brain.read_output(num_actions)
    }
}
