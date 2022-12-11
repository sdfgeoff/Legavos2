use neural::Rnn;
use serde::{Deserialize, Serialize};

use super::controller;
use super::predictor;

#[derive(Serialize, Deserialize)]
struct Buffer {
    prev_state: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct Core {
    pub controller: controller::Controller,
    pub predictor: predictor::Predictor,
}

impl Core {
    pub fn new() -> Self {
        return Core {
            predictor: predictor::Predictor {
                brain: Rnn::new(1000),
                gaussion_approximations: 3,
            },
            controller: controller::Controller {
                brain: Rnn::new(500),
            },
        };
    }

    // Steps the core. This takes the current robot state and the previous set of actions
    // and generates a new set of actions, writes to the memory buffer etc.
    pub fn step(&mut self, state: &[f32], actions: &[f32]) -> Vec<f32> {
        // Figure out what we are going to do next
        let predicted_state = self.predictor.step(state, actions);
        let new_actions = self.controller.step(state, &predicted_state, actions.len());

        new_actions
    }
}
