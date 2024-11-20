/// unifies mouse input and gamepad input
use bevy::prelude::*;

use crate::input::PlayerInput;

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

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
) {
    if let Ok(mut player) = query.get_single_mut() {
        player.rotate_y(player_input.aim_direction.x);
        player.rotate_z(player_input.aim_direction.y);
        player.translation.x += player_input.movement_direction.x;
        player.translation.y += player_input.movement_direction.y;
        player.translation.z += player_input.elevation;
    }
}
