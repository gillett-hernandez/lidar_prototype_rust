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

pub fn player_movement_system(
    mut query: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
    settings: Res<GameSettings>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        transform.rotate_axis(Dir3::Y, -player_input.aim_direction.x);

        transform.rotate_local_z(-player_input.aim_direction.y);

        let x_vec3 = transform.local_x().as_vec3();
        let z_vec3 = transform.local_z().as_vec3();

        transform.translation += settings.movement_speed_factor
            * (x_vec3 * player_input.movement_direction.y
                + z_vec3 * player_input.movement_direction.x);
        transform.translation.y += player_input.elevation * settings.movement_speed_factor;
    }
}
