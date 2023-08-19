use bevy::prelude::*;
use bevy_gltf_collider::get_scene_colliders;
use bevy_rapier3d::prelude::*;

mod camera;
use camera::*;

mod light;
use light::*;

mod balls;
use balls::*;


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States, Default)]
enum ResourcesState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Default, Resource)]
struct GameResources {
    table: Handle<Scene>,
    table_colliders: Vec<(Collider, Transform)>,
    cue: Handle<Scene>
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default().with_physics_scale(1.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_state::<ResourcesState>()
        .add_systems(OnEnter(ResourcesState::Loading), load_resources)
        .add_systems(Update, (check_if_loaded,).run_if(in_state(ResourcesState::Loading)))
        .add_systems(OnEnter(ResourcesState::Loaded), (spawn_light, spawn_table, spawn_balls, spawn_camera))
        .add_systems(Update, (orbit_camera_movement, hit_ball).run_if(in_state(ResourcesState::Loaded)))
        .run();
}

fn load_resources(
    mut commands: Commands, 
    asset_server: Res<AssetServer>
) {
    commands.insert_resource(GameResources {
        table: asset_server.load("models/pool_table.glb#Scene0"),
        cue: asset_server.load("models/cue.glb#Scene0"),
        ..default()
    });
}

fn check_if_loaded(
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

    // get_scene_colliders should be called only once per scene as it will remove the colliders meshes from it
    game_assets.table_colliders = get_scene_colliders(&mut meshes, &mut scene.world)
        .expect("Failed to create monkey colliders");

    println!("Loaded");
    resources_state.set(ResourcesState::Loaded);
}

fn spawn_table(
    mut commands: Commands,
    game_resources: ResMut<GameResources>,
) {
    commands.spawn((
        RigidBody::Fixed,
        //Restitution::coefficient(0.7),
        //Name::new(format!("pool_table")),
        SceneBundle {
            scene: game_resources.table.clone(),
            //transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
    ))
    // Spawn colliders
    .with_children(|parent| {
        for (collider, transform) in game_resources.table_colliders.iter() {
            parent.spawn((
                collider.clone(),
                TransformBundle::from_transform(*transform),
            ));
        }
    });

    commands.spawn(SceneBundle{scene: game_resources.cue.clone(), ..default()})
        .insert(Transform{
            scale: Vec3::splat(60.),
            translation: Vec3::new(-10.0, 40., 100.),
            ..default()
        });
        //.insert(Transform::from_scale(Vec3::splat(100.)));
}
