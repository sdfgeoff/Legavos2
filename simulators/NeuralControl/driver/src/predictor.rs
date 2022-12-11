use neural::Rnn;
use serde::{Deserialize, Serialize};

use super::probability::{gaussian_approximations_from_array, ProbabilityDistributionFunction};

#[derive(Serialize, Deserialize, Debug)]
pub struct Predictor {
    pub brain: Rnn,
    pub gaussion_approximations: usize,
}

impl Predictor {
    pub fn step(&mut self, state: &[f32], actions: &[f32]) -> Vec<f32> {
        let mut all_inputs: Vec<f32> = vec![];
        all_inputs.extend_from_slice(state);
        all_inputs.extend_from_slice(actions);

        self.brain.set_input(&all_inputs);
        self.brain.step();

        let predicted_state = self
            .brain
            .read_output(state.len() * self.gaussion_approximations * 3);

        predicted_state
    }

    fn calculate_loss(
        &mut self,
        prev_state: &[f32],
        prev_actions: &[f32],
        resulting_state: &[f32],
    ) -> f32 {
        // Calculates how well the network predicted a particular state
        // Closer to zero = better

        let predicted_state = self.step(prev_state, prev_actions);

        let mut prediction_distribution =
            gaussian_approximations_from_array(predicted_state, self.gaussion_approximations);
        assert_eq!(prediction_distribution.len(), resulting_state.len());

        // calculate normality
        let normality_score: f32 = prediction_distribution.iter().map(|distribution| distribution.how_normalized() ).sum();

        // Normalize prediction distributions
        for distribution in prediction_distribution.iter_mut() {
            distribution.normalize();
        }

        let accuracy_score: f32 = prediction_distribution.iter().zip(resulting_state.iter()).map(
            |(predicted_state_probability_function, actual_state_item)|
            predicted_state_probability_function.how_likely(*actual_state_item)
        ).sum();
        

        return normality_score + accuracy_score
    }
}
