
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_scene_hook::{SceneHook, HookedSceneBundle};

mod controller;
mod gltf_loader;

use controller::{Controller, step_controller};






fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert(Controller::new());

    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(100.0, 1.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));


    // How many entities?
    // Each leg has three moving parts
    // So call it 13 physics entities per robot.
    // Looks like we can push ~800 entities
    // so probably ~50 robots

    /* Create the bouncing ball. */
    for i in 0..100 {
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.05))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(i as f32 * 0.001, 4.0 + (i as f32)*0.1, 0.0)));
    }

    let gltfs = vec![
        asset_server.load("Robot.glb")
    ];
    commands.insert_resource(gltf_loader::BlenderGltfLoader {
        gltfs
    });
}


fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system(gltf_loader::spawn_gltfs)
        .add_startup_system(startup)
        .add_system(step_controller)
        .run();

}
