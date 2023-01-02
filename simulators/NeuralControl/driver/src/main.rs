use std::env;
use std::mem;

mod controller;
mod memory;
mod network;
mod neural_core;
mod predictor;
mod probability;

struct Main {
    net: network::Network,
    brain: neural_core::Core,

    state_vec: Vec<f32>,
    action_vec: Vec<f32>,

    memory: Vec<memory::MemoryItem>,
}

impl Main {
    fn new(hostname: String) -> Result<Self, std::io::Error> {
        let net = network::Network::new(hostname.clone())?;
        let mut brain = neural_core::Core::new();

        brain.predictor.brain.mutate_weights(1.0);
        brain.controller.brain.mutate_weights(1.0);

        Ok(Self {
            net,
            brain,
            state_vec: vec![],
            action_vec: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            memory: vec![],
        })
    }

    fn iterate(&mut self) -> Result<(), std::io::Error> {
        // When we receive a message from the robot
        match self.net.wait_for_message() {
            Ok(new_message) => self.handle_new_message(new_message),
            Err(err) => {
                // We didn't hear from it for a while. Perhaps the bot doesn't know where we are?
                println!("Error waiting for message. Attempting a ping");
                self.net.send(&network::Message::Ping)
            }
        }
    }

    fn handle_new_message(&mut self, new_message: network::Message) -> Result<(), std::io::Error> {
        match new_message {
            network::Message::State(new_state_vec) => {
                // We can compute a new action vector:
                let new_action_vec = self.brain.step(&new_state_vec, &self.action_vec);

                // And we can compute a new set of actions for the bot
                self.net
                    .send(&network::Message::Action(new_action_vec.clone()))?;

                // We also want to record the results of the previous actions and state
                // for online training:
                let previous_action_vec = mem::replace(&mut self.action_vec, new_action_vec);
                let previous_state_vec = mem::replace(&mut self.state_vec, new_state_vec.clone());

                if previous_state_vec.len() > 0 {
                    self.memory.push(memory::MemoryItem {
                        prev_state: previous_state_vec,
                        prev_actions: previous_action_vec,
                        new_state: new_state_vec,
                    });
                    println!("Memory Size: {}", self.memory.len());
                }
                Ok(())
            }

            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Unhandled Message Type".to_string(),
            )),
        }
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage {} hostname", args[0]);
        std::process::exit(1);
    }
    let hostname = args[1].to_string() + &":42424";

    let mut m = Main::new(hostname)?;

    loop {
        m.iterate()?;

        let mem = &m.memory;
        let mut brain = m.brain.predictor.clone();
        todo!("How to 'reset' the prediction network so it gives consistent scoring? Set it's state to zeros? I guess so")

        let predictor_performance: f32 = mem.iter().map(|memory_item|
            brain.calculate_loss(
                &memory_item.prev_state,
                &memory_item.prev_actions,
                &memory_item.new_state,
            )
        ).sum();

        println!("predictor_performance: {:?}", m.state_vec);
    }
}
