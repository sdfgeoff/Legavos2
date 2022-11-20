mod gltf_loader;
mod rapier_physics;

use bevy::prelude::{App, Plugin};

pub use gltf_loader::gltf_component_parser;


/// Plugin to run hooks associated with spawned scenes.
pub struct GltfComponentLoaderPlugin;
impl Plugin for GltfComponentLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(rapier_physics::body_description_to_builder);
        app.add_system(rapier_physics::collider_description_to_builder);
    }
}