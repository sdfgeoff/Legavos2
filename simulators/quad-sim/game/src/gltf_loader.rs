use bevy::prelude::*;
use bevy::gltf::{Gltf, GltfExtras};


pub struct BlenderGltfLoader {
    pub gltfs: Vec<Handle<Gltf>>
}





pub fn spawn_gltfs(
    mut commands: Commands,
    mut gltf_loader: ResMut<BlenderGltfLoader>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_scene: Res<Assets<Scene>>,
) {
    // if the GLTF has loaded, we can navigate its contents

    gltf_loader.gltfs.retain(|gltf| {
        if let Some(mut raw_asset) = assets_gltf.get_mut(gltf) {
            // Loaded already
            for raw_scene in raw_asset.scenes.iter_mut() {
                if let Some(mut scene) = assets_scene.get_mut(raw_scene) {
                    let world = &mut scene.world;
                    let mut nodes_with_extras = world.query::<(Entity, &GltfExtras)>();
                    for (entity, extras) in nodes_with_extras.iter(&world) {
                        println!("{:?}. {:?}", entity, extras);
                    }
                } else {
                    println!("FATAL: GLTF loaded but scene not loaded?!")
                }
            }

            

            // For testing: spawn it in!
            commands.spawn_bundle(SceneBundle {
                    scene: raw_asset.scenes[0].clone(),
                    ..Default::default()
                });

            return false
        }
        return true // Hopefully it'll be loaded next time
    })
}