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

const PITCH_LOWER_LIMIT: f32 = -FRAC_PI_2;
const PITCH_UPPER_LIMIT: f32 = FRAC_PI_2;

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
    settings: Res<GameSettings>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_axis(Dir3::Y, -player_input.aim_direction.x);

        let (_, _, pitch) = transform.rotation.to_euler(EulerRot::YXZ);
        if pitch < PITCH_UPPER_LIMIT && player_input.aim_direction.y < 0.0
            || pitch > PITCH_LOWER_LIMIT && player_input.aim_direction.y > 0.0
        {
            transform.rotate_local_z(-player_input.aim_direction.y);
        }

        let x_vec3 = transform.local_x().as_vec3();
        let z_vec3 = transform.local_z().as_vec3();

        transform.translation += settings.movement_speed_factor
            * (x_vec3 * player_input.movement_direction.y
                + z_vec3 * player_input.movement_direction.x);
        transform.translation.y += player_input.elevation * settings.movement_speed_factor;
    }
}
