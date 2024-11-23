use std::f32::consts::FRAC_PI_2;

/// unifies mouse input and gamepad input
use bevy::prelude::*;

use crate::{input::PlayerInput, settings::GameSettings};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    spatial: SpatialBundle,
}

impl PlayerBundle {
    pub fn new(spatial: SpatialBundle) -> Self {
        PlayerBundle {
            player: Player,
            spatial,
        }
    }
}

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.05;

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
    settings: Res<GameSettings>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let delta_secs = time.delta_seconds();
        transform.rotate_axis(Dir3::Y, -player_input.aim_direction.x);

        let (_, _, pitch) = transform.rotation.to_euler(EulerRot::YXZ);
        if pitch < PITCH_LIMIT && player_input.aim_direction.y < 0.0
            || pitch > -PITCH_LIMIT && player_input.aim_direction.y > 0.0
        {
            transform.rotate_local_z(-player_input.aim_direction.y );
        }

        let x_vec3 = -transform.local_z().as_vec3().cross(Vec3::Y).normalize();
        let z_vec3 = transform.local_x().as_vec3().cross(Vec3::Y).normalize();

        transform.translation += settings.movement_speed_factor
            * delta_secs
            * (x_vec3 * player_input.movement_direction.y
                + z_vec3 * player_input.movement_direction.x);
        transform.translation.y +=
            player_input.elevation * settings.movement_speed_factor * delta_secs;
    }
}
