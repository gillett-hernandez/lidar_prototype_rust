use std::f32::consts::{PI, TAU};

/// unifies mouse input and gamepad input
use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::settings::GameSettings;

#[derive(Default, Copy, Clone, Debug)]
pub enum FiringMode {
    #[default]
    None, // not firing
    Firing,
    Burst,
}

#[derive(Copy, Clone, Debug)]
pub enum PressedStatus {
    NotPressed,
    JustPressed,
    Held,
    JustReleased,
}

#[derive(Resource)]
pub struct PlayerInput {
    pub movement_direction: Vec2,
    pub aim_direction: Vec2,
    pub firing_mode: FiringMode,
    pub fire_trigger: PressedStatus,
    pub burst_trigger: PressedStatus,
}

pub fn player_input_system(
    mut player_input: ResMut<PlayerInput>,
    mut mouse_movements: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    // gamepad_buttons: Res<ButtonInput<GamepadButton>>,
    // gamepad_axis: Res<Axis<GamepadAxis>>,
    game_settings: Res<GameSettings>,
) {
    let mut move_direction = Vec2::ZERO;

    // TODO: implement gamepad support

    for key in keyboard.get_pressed() {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                move_direction.y += 1.0;
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                move_direction.x -= 1.0;
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                move_direction.y -= 1.0;
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                move_direction.x += 1.0;
            }
            _ => {}
        }
    }

    if move_direction.length_squared() > 0.0 {
        move_direction = move_direction.normalize();
    }

    player_input.movement_direction = move_direction;

    player_input.aim_direction = Vec2::ZERO;

    for mouse_movement in mouse_movements.read() {
        let (dtheta, dphi) = (
            mouse_movement.delta.x * game_settings.sensitivity.x,
            mouse_movement.delta.y * game_settings.sensitivity.y,
        );

        player_input.aim_direction.x += dtheta;
        player_input.aim_direction.x %= TAU;
        player_input.aim_direction.y += dphi;
        // player_input.aim_direction.y = player_input.aim_direction.y.clamp(0.0, PI);
    }

    for mouse_button in mouse_button_input.get_pressed() {
        match mouse_button {
            MouseButton::Left => {
                player_input.fire_trigger = PressedStatus::Held;
            }
            MouseButton::Right => {
                player_input.burst_trigger = PressedStatus::Held;
            }
            _ => {}
        }
    }
    for mouse_button in mouse_button_input.get_just_pressed() {
        match mouse_button {
            MouseButton::Left => {
                player_input.fire_trigger = PressedStatus::JustPressed;
            }
            MouseButton::Right => {
                player_input.burst_trigger = PressedStatus::JustPressed;
            }
            _ => {}
        }
    }
    for mouse_button in mouse_button_input.get_just_released() {
        match mouse_button {
            MouseButton::Left => {
                player_input.fire_trigger = PressedStatus::JustReleased;
            }
            MouseButton::Right => {
                player_input.burst_trigger = PressedStatus::JustReleased;
            }
            _ => {}
        }
    }
}
