use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera::OrbitCamera;

pub const BALL_RADIUS: f32 = 0.025;
const BALL_FRADIUS: f32 = BALL_RADIUS + BALL_RADIUS * 0.001;
//const BALL_DIAMETER: f32 = BALL_RADIUS * 2.0;
const BALL_FDIAMETER: f32 = BALL_FRADIUS * 2.0;
const BALLS_TRIANGLE_BASE: Vec3 = Vec3::new(0.0, BALL_FRADIUS, -0.55);
pub const CUEBALL_BASE_POSITION:Vec3 = Vec3::new(0., BALL_FRADIUS, 0.64);

#[derive(Component)]
pub struct Ball {
    pub number: u8,
}

impl Ball {
    pub fn from_num(num: u8) -> Self {
        Self { number: num }
    }
}

#[derive(Component)]
pub struct CueBall {}
impl Default for CueBall {
    fn default() -> Self {
        Self {  }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]
pub enum BallsState {
    #[default]
    Stopped,
    Moving
}

#[derive(Bundle)]
struct BallBundle {
    pbr_bundle: PbrBundle,
    rigid_body: RigidBody,
    collider: Collider,
    mass_proporties: ColliderMassProperties,
    damping: Damping,
    gravity: GravityScale,
    restitution: Restitution,
    ccd: Ccd,
    sleeping: Sleeping
}

impl BallBundle {
    pub fn new(
        meshes: &mut ResMut<'_, Assets<Mesh>>,
    ) -> Self {
        let mut sleep_conf = Sleeping::default();
        sleep_conf.sleeping = true;
        Self { 
            pbr_bundle: PbrBundle { 
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    sectors: 64,
                    stacks: 64,
                    radius: BALL_RADIUS
                })),
                ..default()
            },
            rigid_body: RigidBody::Dynamic, 
            collider: Collider::ball(BALL_RADIUS),
            mass_proporties: ColliderMassProperties::Mass(0.155922377),
            damping: Damping { linear_damping: 0.3, angular_damping: 0.3 },
            gravity: GravityScale(1.),
            restitution: Restitution::coefficient(0.95),
            ccd: Ccd::enabled(),
            sleeping: sleep_conf
        }
    }

    pub fn black(mut self, materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        self.pbr_bundle.material = materials.add(StandardMaterial {
            base_color: Color::hex("#010101").unwrap(),
            perceptual_roughness: 0.,
            reflectance: 1.,
            ..default()
        });
        self
    }

    pub fn white(mut self, materials: &mut ResMut<Assets<StandardMaterial>>) -> Self {
        self.pbr_bundle.material = materials.add(StandardMaterial {
            base_color: Color::hex("#F0F0A0").unwrap(),
            perceptual_roughness: 0.,
            reflectance: 1.,
            ..default()
        });
        self
    }
}

pub fn spawn_balls(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut spawn_ball = |pos: Vec3, num: u8| {
        commands.spawn((
            BallBundle::new(&mut meshes).white(&mut materials),
            Ball::from_num(num)
        ))
        .insert(Transform { translation: pos, ..default() });
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
            spawn_ball(Vec3::new(shift_x, BALL_FRADIUS, shift_z) ,counter);
        }
        columns += 1;
    }

    /* Cue Ball */
    commands.spawn((
        BallBundle::new(&mut meshes).black(&mut materials),
        Ball::from_num(0),
        CueBall::default()
    ))
    .insert(Transform::from_translation(CUEBALL_BASE_POSITION));
}

pub fn hit_ball(
    q_cam: Query<&OrbitCamera>,
    q_ball: Query<(Entity, With<CueBall>)>,
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut balls_state: ResMut<NextState<BallsState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let orbit_cam = q_cam.get_single().unwrap();
        let vision_direction = orbit_cam.to_decart_xz() / Vec3::splat(orbit_cam.radius);
        for (cue_ball, ()) in q_ball.iter() {
            commands.entity(cue_ball).insert(ExternalImpulse {
                impulse: -vision_direction * 0.85,
                torque_impulse: Vec3::splat(0.0),
            });
        }
        balls_state.set(BallsState::Moving);
    }
}

pub fn moving_balls_checker(
    balls: Query<(&Sleeping, With<Ball>)>,
    mut balls_state: ResMut<NextState<BallsState>>,
) {
    let mut balls_in_move = 0u8;
    balls.for_each(|(ball, ())| {
        if !ball.sleeping {
            balls_in_move += 1;
        }
    });
    println!("balls in move = {}", balls_in_move);
    if balls_in_move == 0 { balls_state.set(BallsState::Stopped) }
}

pub fn pocket_hole_collector(
    balls: Query<(Entity, &Transform), With<Ball>>,
    mut commands: Commands,
) {
    balls.for_each(|(ball, transform)| {
        if transform.translation.y < 0.0 {
           commands.entity(ball).despawn();
        }
    });
}