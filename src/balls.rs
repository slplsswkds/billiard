use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera::OrbitCamera;

const BALL_RADIUS: f32 = 0.025;
const BALL_FRADIUS: f32 = BALL_RADIUS + BALL_RADIUS * 0.001;
//const BALL_DIAMETER: f32 = BALL_RADIUS * 2.0;
const BALL_FDIAMETER: f32 = BALL_FRADIUS * 2.0;
const BALLS_TRIANGLE_BASE: Vec3 = Vec3::new(0.0, BALL_FRADIUS, -0.55);

#[derive(Component)]
pub struct Ball {
    _number: u8,
}

impl Ball {
    pub fn from_num(num: u8) -> Self {
        Self { _number: num }
    }
}


#[derive(Component)]
pub struct CueBall {}
impl Default for CueBall {
    fn default() -> Self {
        Self {  }
    }
}

pub fn spawn_balls(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn_ball = |pos: Vec3, num: u8| {
        commands.spawn((
            Collider::ball(BALL_RADIUS),
            RigidBody::Dynamic,
            ColliderMassProperties::Density(1.),
            Damping { linear_damping: 0.2, angular_damping: 0.2 },
            GravityScale(1.),
            Restitution::coefficient(0.95),
            Ccd::enabled(),
            PbrBundle{
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    sectors: 64,
                    stacks: 64,
                    radius: BALL_RADIUS
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::hex("#F0F0A0").unwrap(),
                    perceptual_roughness: 0.,
                    reflectance: 1.,
                    ..default()
                }),
                ..default()
            },
            Ball::from_num(num)
        ))
        .insert(Transform { 
            translation: pos,
            ..default()
        });
    };

    let mut columns = 1u8;
    let mut counter: u8 = 0;
    let mut shift_x;
    let mut shift_z;
    for row in 0..5 {
        shift_z = BALLS_TRIANGLE_BASE.z - BALL_FDIAMETER * row as f32;
        for col in 0..columns {
            counter += 1;
            if columns % 2 == 0 {
                shift_x = BALLS_TRIANGLE_BASE.x + BALL_FRADIUS - BALL_FRADIUS * columns as f32 + BALL_FDIAMETER * col as f32;
            } else {
                shift_x = BALLS_TRIANGLE_BASE.x - BALL_FRADIUS * columns as f32 + BALL_FRADIUS + BALL_FDIAMETER * col as f32;
            }
            //println!("{}, {}, {}", counter, row, col);
            spawn_ball(Vec3::new(shift_x, 0.0, shift_z) ,counter);
        }
        columns += 1;
    }

    commands.spawn(
        PbrBundle{
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                sectors: 64,
                stacks: 64,
                radius: BALL_RADIUS
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("#010101").unwrap(),
                perceptual_roughness: 0.,
                reflectance: 1.,
                ..default()
            }),
            transform: Transform::from_xyz(0., BALL_FRADIUS, 0.64),
            ..default()
        }
    )
    .insert((
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        ColliderMassProperties::Density(1.),
        Damping { linear_damping: 0.2, angular_damping: 0.2 },
        GravityScale(1.),
        Restitution::coefficient(0.95),
        Ccd::enabled()
    ))
    .insert(CueBall::default());
}

pub fn hit_ball(
    q_cam: Query<&OrbitCamera>,
    q_ball: Query<(Entity, With<CueBall>)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let orbit_cam = q_cam.get_single().unwrap();
        let vision_direction = orbit_cam.to_decart_xz() / Vec3::splat(orbit_cam.radius);
        for (cue_ball, ()) in q_ball.iter() {
            commands.entity(cue_ball).insert(ExternalImpulse {
                impulse: -vision_direction * 0.00025,
                torque_impulse: Vec3::splat(0.0),
            });
            println!("succes!");
        }
    }
}