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
    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_axis(Dir3::Y, -player_input.aim_direction.x);

        transform.rotate_local_z(-player_input.aim_direction.y);

        transform.translation.x += player_input.movement_direction.x;
        transform.translation.y += player_input.movement_direction.y;
        transform.translation.z += player_input.elevation;
    }
}
