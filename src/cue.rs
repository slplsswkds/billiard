use bevy::prelude::*;
use crate::balls;
use crate::resources::GameResources;
use crate::balls::{CueBall, BallsState};
use crate::camera::OrbitCamera;
use bevy_rapier3d::prelude::ExternalImpulse;

const MAX_POWER_CUE_DISTANCE: f32 = 10.0 * balls::BALL_RADIUS;
const DEFAULT_CUE_DISTANCE: f32 = balls::BALL_FRADIUS;

#[derive(Component)]
pub struct Cue {
    pub radius: f32,
    pub height: f32,
    power: f32,
}

impl Default for Cue {
    fn default() -> Self {
        Self {
            radius: balls::BALL_RADIUS,
            height: balls::BALL_RADIUS,
            power: 0f32
        }
    }
}

impl Cue {
    fn increase_power(&mut self, delta_secs: f32) {
        self.power += delta_secs * 0.5;
        if self.power > 1. { 
            self.power = 1. 
        }
        println!("Power = {}", self.power);
    }
}

pub fn spawn_cue(
    mut commands: Commands,
    game_resources: ResMut<GameResources>,
) {
    commands
    .spawn(SceneBundle{scene: game_resources.cue.clone(), ..default()})
    .insert(Transform{
        translation: balls::CUEBALL_BASE_POSITION,
        ..default()
    })
    .insert(Cue::default());
}

pub fn orbit_cue(
    mut q_cam: Query<&mut OrbitCamera>,
    mut q_cue: Query<(&mut Transform, &mut Cue)>,
    q_cue_ball: Query<&Transform, (With<CueBall>, Without<Cue>)>
) {
    let cam = q_cam.get_single_mut().unwrap();
    let (mut cue_transform, mut cue) = q_cue.get_single_mut().unwrap();
    match q_cue_ball.get_single() {
        Err(_) => {},
    Ok(cue_ball) => {
            cue.radius = DEFAULT_CUE_DISTANCE + cue.power * MAX_POWER_CUE_DISTANCE;
            let x = cue.radius * cam.angle.cos();
            let z = cue.radius * cam.angle.sin();
            cue_transform.translation.x = cue_ball.translation.x + x;
            cue_transform.translation.z = cue_ball.translation.z + z;
            cue_transform.translation.y = cue.height/1.75;
            cue_transform.look_at(cue_ball.translation, Vec3::Y);
        }
    }
}

pub fn hit_ball(
    q_cam: Query<&OrbitCamera>,
    q_ball: Query<(Entity, With<CueBall>)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut balls_state: ResMut<NextState<BallsState>>,
    mut q_cue: Query<&mut Cue>,
    time: Res<Time>,
) {
    let mut cue = q_cue.get_single_mut().unwrap();

    if keys.pressed(KeyCode::Space) {
        cue.increase_power(time.delta_seconds());
    }
    if keys.just_released(KeyCode::Space) {
        let orbit_cam = q_cam.get_single().unwrap();
        let vision_direction = orbit_cam.to_decart_xz() / Vec3::splat(orbit_cam.radius);
        for (cue_ball, ()) in q_ball.iter() {
            commands.entity(cue_ball).insert(ExternalImpulse {
                impulse: -vision_direction * cue.power,
                torque_impulse: Vec3::splat(0.0),
            });
        }
        balls_state.set(BallsState::Moving);
        cue.power = 0.0;
    }
}
