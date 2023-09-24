use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod resources;
mod camera;
mod light;
mod balls;
mod table;
mod spatial_controller;

use resources::*;
use camera::*;
use light::*;
use balls::*;
use table::*;
use spatial_controller::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_state::<ResourcesState>()
        .add_systems(OnEnter(ResourcesState::Loading), load_resources)
        .add_systems(Update, (check_if_loaded).run_if(in_state(ResourcesState::Loading)))
        .add_systems(OnEnter(ResourcesState::Loaded), (spawn_light, spawn_table, spawn_balls, spawn_camera, init_game_progress_info))
        .add_systems(Update, (orbit_camera_movement, hit_ball, debug_game_progress_info).run_if(in_state(ResourcesState::Loaded)))
        .run();
}

