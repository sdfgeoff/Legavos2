use bevy::ecs::{
    system::{EntityCommands},
    world::EntityRef,
};
use bevy::gltf::{GltfExtras};
use bevy::prelude::*;
use serde::{Deserialize, Deserializer};

use serde_json;

use super::rapier_physics;

pub fn object_empty_as_none<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    for<'a> T: Deserialize<'a>,
{
    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct Empty {}

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    enum Aux<T> {
        Empty(Empty),
        Null,
        T(T),
    }

    match Deserialize::deserialize(deserializer)? {
        Aux::T(t) => Ok(Some(t)),
        Aux::Empty(_) | Aux::Null => Ok(None),
    }
}

#[derive(Debug, Deserialize)]
struct Empty {}

#[derive(Debug, Deserialize)]
struct Components {
    #[serde(deserialize_with = "object_empty_as_none")]
    rapier_rigid_body: Option<rapier_physics::RigidBodyDescription>,

    #[serde(deserialize_with = "object_empty_as_none")]
    rapier_collider_description: Option<rapier_physics::ColliderDescription>,
}

pub fn gltf_component_parser(entity: &EntityRef, commands: &mut EntityCommands) {
    if let Some(extras) = entity.get::<GltfExtras>() {
        match serde_json::from_str::<Components>(&extras.value) {
            Ok(parsed) => {
                if let Some(p) = parsed.rapier_rigid_body {
                    commands.insert(p);
                }
                if let Some(p) = parsed.rapier_collider_description {
                    commands.insert(p);
                }
            }
            Err(err) => {
                error!("Failed parsing component. Err: {}", err);
            }
        }
    }
}
