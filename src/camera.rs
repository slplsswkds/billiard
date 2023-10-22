use bevy::prelude::*;
use bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle;
use bevy::input::mouse::MouseMotion;

use crate::balls::CueBall;

#[derive(Component)]
pub struct OrbitCamera {
    pub radius: f32,
    pub angle: f32,
    pub height: f32,
}

impl OrbitCamera {
    pub fn to_decart_xz(&self) -> Vec3 {
        let mut decart = Vec3::splat(0.0);
        decart.x = self.radius * self.angle.cos();
        decart.z = self.radius * self.angle.sin();
        //decart.y = self.radius / 3.0;
        decart
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self{
            radius: 1.25,
            angle: -67.5,
            height: 0.3,
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                ..default()
            },
        transform: Transform::from_xyz(0.0, 0.5, 1.89).looking_at(Vec3::new(0.0, -0.25, 0.0), Vec3::Y),
        ..default()
        },
        TemporalAntiAliasBundle::default(),
        OrbitCamera::default(),
    ));
}

pub fn orbit_camera_movement(
    mut q_cam: Query<(&mut Transform, &mut OrbitCamera)>,
    mut motion_evr: EventReader<MouseMotion>,
    q_cue_ball: Query<&Transform, (With<CueBall>, Without<OrbitCamera>)>
) {
    let cue_ball_pos = match q_cue_ball.get_single() {
        Ok(pos) => pos.translation,
        Err(_) => Vec3::splat(0.0),
    };
    for ev in motion_evr.iter() {
        for (mut transform, mut orbit_cam) in q_cam.iter_mut() {
            orbit_cam.angle += ev.delta.x * 0.0025; // 0.001
            if orbit_cam.angle >= 360.0 { orbit_cam.angle -= 360.0 } 
            else if orbit_cam.angle <= -360.0 { orbit_cam.angle += 360.0 } 
            let x = orbit_cam.radius * orbit_cam.angle.cos();
            let z = orbit_cam.radius * orbit_cam.angle.sin();
            transform.translation.x = cue_ball_pos.x + x;
            transform.translation.z = cue_ball_pos.z + z;
            transform.translation.y = orbit_cam.height;
            transform.look_at(cue_ball_pos, Vec3::Y);
        }
    }
}

pub fn look_at_cue_ball(
    mut q_cam: Query<&mut Transform, With<OrbitCamera>>,
    q_cue_ball: Query<&Transform, (With<CueBall>, Without<OrbitCamera>)>
) {
    let mut camera = q_cam.get_single_mut().unwrap();
    match q_cue_ball.get_single() {
        Err(_) => (),
        Ok(cue_ball_transform) => {
            camera.look_at(cue_ball_transform.translation, Vec3::Y);
        }
    }
}
