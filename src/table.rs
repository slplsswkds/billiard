use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::resources::GameResources;

pub fn spawn_table(
    mut commands: Commands,
    game_resources: ResMut<GameResources>,
) {
    commands.spawn((
        RigidBody::Fixed,
        SceneBundle {
            scene: game_resources.table.clone(),
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
}
