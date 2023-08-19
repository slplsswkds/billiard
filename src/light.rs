use bevy::prelude::*;
use std::f32::consts::PI;

pub fn spawn_light(
    mut commands: Commands
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100000.,
            shadows_enabled: true,
            color: Color::rgba(1., 1., 1., 0.0),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: bevy::pbr::CascadeShadowConfigBuilder {
            minimum_distance: 0.1,
            maximum_distance: 3.0,
            first_cascade_far_bound: 1.75,
            ..default()
        }
        .into(),
        ..default()
    });
}