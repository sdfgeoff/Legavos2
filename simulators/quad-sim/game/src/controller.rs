use bevy::prelude::*;
use neural::Rnn;

#[derive(Component)]
pub struct Controller {
    controller_model: Rnn,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            controller_model: Rnn::new(100),
        }
    }
}

pub fn step_controller(mut query: Query<&mut Controller>) {
    for mut controller in query.iter_mut() {
        controller.controller_model.state[0] = 1.0;
        controller.controller_model.step();
    }
}
