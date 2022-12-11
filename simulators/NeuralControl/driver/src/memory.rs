use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MemoryItem {
    pub prev_state: Vec<f32>,
    pub prev_actions: Vec<f32>,
    pub new_state: Vec<f32>,
}
