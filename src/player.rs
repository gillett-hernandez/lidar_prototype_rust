use std::f32::consts::{PI, TAU};

/// unifies mouse input and gamepad input
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::{input::PlayerInput, settings::GameSettings};

#[derive(Component)]
pub struct Player;

pub fn player_rotation_sync_system(
    mut query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
) {
    for mut player in query.get_single_mut() {
        player.rotate_z(player_input.aim_direction.x);
        player.rotate_y(player_input.aim_direction.y);
    }
}
