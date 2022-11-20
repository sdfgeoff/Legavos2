use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};

use bevy_gltf_component_loader::{GltfComponentLoaderPlugin, gltf_component_parser};

mod controller;

use controller::{step_controller, Controller};



fn spawn_gltf_scene(commands: &mut Commands, asset_server: &Res<AssetServer>, scene: &str, transform: Transform) {
    commands.spawn(HookedSceneBundle {
        scene: SceneBundle {
            scene: asset_server.load(scene),
            transform,
            ..default()
        },
        hook: SceneHook::new(|entity, commands| {
            gltf_component_parser(entity, commands)
        }),
    }).insert(Name::new(scene.to_string()));

}



fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Controller::new()).insert(Name::new("ControllerTest"));

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-0.5, 0.5, 1.5).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // How many entities?
    // Each leg has three moving parts
    // So call it 13 physics entities per robot.
    // Looks like we can push ~800 entities
    // so probably ~50 robots

    /* Create the bouncing ball. */
    // for i in 0..100 {
    //     commands
    //         .spawn_empty()
    //         .insert(RigidBody::Dynamic)
    //         .insert(Collider::ball(0.05))
    //         .insert(Restitution::coefficient(0.7))
    //         .insert(TransformBundle::from(Transform::from_xyz(
    //             i as f32 * 0.001,
    //             4.0 + (i as f32) * 0.1,
    //             0.0,
    //         )));
    // }
    
    spawn_gltf_scene(&mut commands, &asset_server, "Robot.glb#Scene0", Transform::from_xyz(0.0, 0.1, 0.0));
    
    spawn_gltf_scene(&mut commands, &asset_server, "World.glb#Scene0", Transform::from_xyz(0.0, 0.0, 0.0));
    
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(HookPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GltfComponentLoaderPlugin)
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(startup)
        .add_system(step_controller)
        .run();
}
