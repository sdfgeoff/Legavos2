use neural::Rnn;
use serde::{Deserialize, Serialize};

use super::probability::{gaussian_approximations_from_array, ProbabilityDistributionFunction};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

    pub fn calculate_loss(
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

        // calculate normality. Numbers close to zero indicate that this is more normalized
        let normality_score: f32 = prediction_distribution.iter().map(|distribution| distribution.how_normalized() ).sum();

        println!("normality: {:?}", normality_score);

        // Normalize prediction distributions
        for distribution in prediction_distribution.iter_mut() {
            distribution.normalize();
        }

        let accuracy_score: f32 = prediction_distribution.iter().zip(resulting_state.iter()).map(
            |(predicted_state_probability_function, actual_state_item)| {
                // Ranges between 0 and (theoretically) 1
                // where zero is impossible and 1 is guaranteed
                let how_likely = predicted_state_probability_function.how_likely(*actual_state_item);

                // Now it ranges between 0 when the guess was perfect and 1 when it wasn't.
                1.0 - how_likely
            }
        ).sum();
        

        return normality_score * 0.01 + accuracy_score
    }
}
