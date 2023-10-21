use bevy::prelude::*;
use crate::balls;
use crate::resources::GameResources;
use crate::balls::CueBall;
use crate::camera::OrbitCamera;

#[derive(Component)]
pub struct Cue {
    pub radius: f32,
    pub height: f32,
}

impl Default for Cue {
    fn default() -> Self {
        Self {
            radius: balls::BALL_RADIUS,
            height: balls::BALL_RADIUS
        }
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
    mut q_cue: Query<(&mut Transform, &Cue)>,
    q_cue_ball: Query<&Transform, (With<CueBall>, Without<Cue>)>
) {
    let cam = q_cam.get_single_mut().unwrap();
    let (mut cue_transform, cue) = q_cue.get_single_mut().unwrap();
    match q_cue_ball.get_single() {
        Err(_) => {},
    Ok(cue_ball) => {
            let x = cue.radius * cam.angle.cos();
            let z = cue.radius * cam.angle.sin();
            cue_transform.translation.x = cue_ball.translation.x + x;
            cue_transform.translation.z = cue_ball.translation.z + z;
            cue_transform.translation.y = cue.height/1.75;
            cue_transform.look_at(cue_ball.translation, Vec3::Y);
        }
    }
}
