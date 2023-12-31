use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::pbr::DirectionalLightShadowMap;

mod resources;
mod camera;
mod light;
mod balls;
mod table;
mod spatial_controller;
mod cue;

use resources::*;
use camera::*;
use light::*;
use balls::*;
use table::*;
use spatial_controller::*;
use cue::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DirectionalLightShadowMap { size: 4096 })

        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_state::<ResourcesState>()
        .add_systems(OnEnter(ResourcesState::Loading), load_resources)
        .add_systems(Update, (check_if_loaded)
            .run_if(in_state(ResourcesState::Loading)))
        .add_systems(OnEnter(ResourcesState::Loaded), (spawn_light, spawn_table, spawn_balls, spawn_camera, spawn_cue, init_game_progress_info))
        .add_state::<BallsState>()
        .add_systems(Update, (orbit_cue, orbit_camera_movement, hit_ball, cue_ball_choose_position)
            .run_if(in_state(BallsState::Stopped))
            .run_if(in_state(ResourcesState::Loaded)))
        .add_systems(Update, (moving_balls_checker, pocket_hole_collector, look_at_cue_ball)
            .run_if(in_state(BallsState::Moving)))
        .add_systems(Update, (cue_ball_choose_position, look_at_cue_ball,pocket_hole_collector)
            .run_if(in_state(BallsState::Manual)))
        .run();
}