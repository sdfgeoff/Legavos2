use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::geometry::SharedShape;
use bevy_rapier3d::rapier::na::Point3;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Default, Component, Serialize, Deserialize, Debug)]
#[reflect(Component)]
/// A RigidBodyDescription is only present until the body_description_to_builder system runs,
/// upon which it is converted to a rapier::dynamics::RigidBodyBuilder with matching properties.
/// Then the bevy rapier plugin converts that into a RigidBodyHandle component for the purpose
/// of actually simulating it.
pub struct RigidBodyDescription {
    /// Because we can't export enums yet, this is encoded as a u8
    ///  0 => dynamic
    ///  1 => static
    ///  2 => kinematic position based
    ///  3 => kinematic velocity based
    #[serde(default)]
    pub body_status: u8,

    /// Damp the rotation of the body
    #[serde(default)]
    pub damping_angular: f32,
    // /// Damp the linear of the body
    #[serde(default)]
    pub damping_linear: f32,

    // /// Enable continous collision detection - good for fast moving objects but increases
    // /// processor load
    #[serde(default)]
    pub ccd_enable: u8,

    // /// Allow the physics engine to "sleep" the body when it's velocity is low. This helps
    // /// save processing time if there are lots of nearly-static-bodies
    #[serde(default)]
    pub sleep_allow: u8,

    #[serde(default)]
    pub lock_translation: [u8; 3],

    #[serde(default)]
    pub lock_rotation: [u8; 3],
}

/// Converts a RigidBodyDescription into a rapier::dynamics::RigidBodyBuilder. This allows
/// RigidBodyBuilders to be created from a file using bevies Reflection system and scene format
pub fn body_description_to_builder(
    mut commands: Commands,
    body_desc_query: Query<(&RigidBodyDescription, Entity)>,
) {
    for (body_desc, entity) in body_desc_query.iter() {
        commands.entity(entity).remove::<RigidBodyDescription>();

        //updated body_status to store updated RigidBody types

        let body_status = match body_desc.body_status {
            0 => RigidBody::Dynamic,
            1 => RigidBody::Fixed,
            2 => RigidBody::KinematicPositionBased,
            3 => RigidBody::KinematicVelocityBased,
            _ => panic!("Unknown rapier body status"),
        };

        //updated lock flags to store LockedAxes to how they are used in the newest version of rapier

        let lock_flags = {
            let mut flags = LockedAxes::empty();
            if body_desc.lock_translation[0] != 0 {
                flags.insert(LockedAxes::TRANSLATION_LOCKED_X);
            }
            if body_desc.lock_translation[1] != 0 {
                flags.insert(LockedAxes::TRANSLATION_LOCKED_Y);
            }
            if body_desc.lock_translation[2] != 0 {
                flags.insert(LockedAxes::TRANSLATION_LOCKED_Z);
            }
            if body_desc.lock_rotation[0] != 0 {
                flags.insert(LockedAxes::ROTATION_LOCKED_X);
            }
            if body_desc.lock_rotation[1] != 0 {
                flags.insert(LockedAxes::ROTATION_LOCKED_Y);
            }
            if body_desc.lock_rotation[2] != 0 {
                flags.insert(LockedAxes::ROTATION_LOCKED_Z);
            }

            flags
        };

        //new method of inserting rigid bodies into entities
        if body_desc.ccd_enable > 0 {
            commands
                .entity(entity)
                .insert(body_status)
                .insert(lock_flags)
                .insert(Damping {
                    linear_damping: body_desc.damping_linear,
                    angular_damping: body_desc.damping_angular,
                })
                .insert(Ccd::enabled());
        } else {
            commands
                .entity(entity)
                .insert(body_status)
                .insert(lock_flags)
                .insert(Damping {
                    linear_damping: body_desc.damping_linear,
                    angular_damping: body_desc.damping_angular,
                })
                .insert(Ccd::disabled());
        }
    }
}

#[derive(Reflect, Default, Component, Serialize, Deserialize, Debug)]
#[reflect(Component)]
pub struct ColliderDescription {
    #[serde(default)]
    friction: f32,

    #[serde(default)]
    restitution: f32,

    #[serde(default)]
    is_sensor: bool,

    #[serde(default)]
    density: f32,

    /// At the moment, you can't use an enum with bevy's Reflect derivation.
    /// So instead we're doing this the old fashioned way.
    ///
    /// collider_shape = 0: Sphere collider
    ///     collider_shape_data: f32 = radius
    #[serde(default)]
    collider_shape: u8,
}

pub fn collider_description_to_builder(
    mut commands: Commands,
    assets_mesh: Res<Assets<Mesh>>,
    collider_desc_query: Query<(&ColliderDescription, Entity, &Children)>,
    mesh_entities: Query<(&Handle<Mesh>, With<Parent>)>
) {
    // println!("THERE!");
    for (collider_desc, entity, children) in collider_desc_query.iter() {
        // println!("HERE!");
        commands.entity(entity).remove::<ColliderDescription>();

        let mesh_handle_o: Option<(&Handle<Mesh>, ())> = children.iter()
            .find_map(|c| mesh_entities.get(*c).ok());
        
        let mesh_handle: &Handle<Mesh> = mesh_handle_o.expect("COuld not find child of collider with mesh").0;
        
        

        let mesh = assets_mesh.get(mesh_handle).expect("Entity has mesh handle for non existing mesh");
        let aabb = mesh.compute_aabb().expect("Failed to generate AABB");

        let centroid = aabb.center;// Vec3::new(0.0, 0.0, 0.0);
        let half_extents = aabb.half_extents;//Vec3::new(0.01, 0.01, 0.01);

        println!("Shape: {}", collider_desc.collider_shape);
        let shape = match collider_desc.collider_shape {
            0 => {
                // Sphere
                let radius = f32::max(half_extents.x, f32::max(half_extents.y, half_extents.z));
                println!("Radius: {}", radius);
                SharedShape::ball(radius)
            }
            1 => {
                // Capsule
                let half_height = half_extents.y;
                let radius = f32::max(half_extents.x, half_extents.z);
                SharedShape::capsule(
                    Point3::new(0.0, 0.0, half_height),
                    Point3::new(0.0, 0.0, -half_height),
                    radius,
                )
            }
            2 => {
                // Box
                SharedShape::cuboid(half_extents.x, half_extents.y, half_extents.z)
            } //fixed typo
            _ => panic!("Unknown collider shape"),
        };

        //new method for adding inserting Colliders and their various properties to entities
        //fixed an issue where colliders were not added to rigid bodies
        commands.entity(entity).with_children(|children| {
            children
                .spawn_empty()
                .insert(Collider::from(shape))
                // .insert(Sensor(collider_desc.is_sensor))
                .insert(Friction {
                    coefficient: collider_desc.friction,
                    ..Default::default()
                })
                .insert(Restitution {
                    coefficient: collider_desc.restitution,
                    ..Default::default()
                })
                .insert(Transform::from_xyz(centroid.x, centroid.y, centroid.z))
                // TODO:
                .insert(ColliderMassProperties::Density(0.1)); // collider_desc.density
        });
    }
}
