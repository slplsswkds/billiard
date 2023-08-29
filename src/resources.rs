use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_gltf_collider::get_scene_colliders;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]
pub enum ResourcesState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Default, Resource)]
pub struct GameResources {
    pub table: Handle<Scene>,
    pub table_colliders: Vec<(Collider, Transform)>,
    pub cue: Handle<Scene>
}

pub fn load_resources(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(GameResources {
        table: asset_server.load("models/pool_table.glb#Scene0"),
        cue: asset_server.load("models/cue.glb#Scene0"),
        ..default()
    });
}

pub fn check_if_loaded(
    mut scenes: ResMut<Assets<Scene>>,
    mut game_assets: ResMut<GameResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut resources_state: ResMut<NextState<ResourcesState>>,
) {
    let scene = if let Some(scene) = scenes.get_mut(&game_assets.table) {
        scene
    } else {
        return;
    };

    game_assets.table_colliders = get_scene_colliders(&mut meshes, &mut scene.world)
        .expect("Failed to create table colliders");

    println!("Loaded");
    resources_state.set(ResourcesState::Loaded);
}