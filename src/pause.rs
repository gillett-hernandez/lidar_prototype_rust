use std::time::Duration;

use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::gamestate::GameState;

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor_options.visible = false;
}

fn cursor_ungrab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();

    primary_window.cursor_options.grab_mode = CursorGrabMode::None;
    primary_window.cursor_options.visible = true;
}

#[derive(Resource, DerefMut, Deref)]
struct PauseDebounceTimer(Timer);

fn pause_menu_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(&Name, &Gamepad)>,
    time: Res<Time>,
    mut pause_debounce_timer: ResMut<PauseDebounceTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let esc_pressed = keyboard_input.just_pressed(KeyCode::Escape);
    let start_pressed = gamepads
        .iter()
        .any(|(_, gamepad)| gamepad.just_pressed(GamepadButton::Start));
    if pause_debounce_timer.tick(time.delta()).finished() && (esc_pressed || start_pressed) {
        next_state.set(GameState::InGame);
        pause_debounce_timer.reset();
    }
}

fn pause_input_handler(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<(&Name, &Gamepad)>,
    time: Res<Time>,
    mut pause_debounce_timer: ResMut<PauseDebounceTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let esc_pressed = keyboard_input.just_pressed(KeyCode::Escape);
    let start_pressed = gamepads
        .iter()
        .any(|(_, gamepad)| gamepad.just_pressed(GamepadButton::Start));
    if pause_debounce_timer.tick(time.delta()).finished() && (esc_pressed || start_pressed) {
        next_state.set(GameState::Paused);
        pause_debounce_timer.reset();
    }
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pause_input_handler.run_if(in_state(GameState::InGame)),
                pause_menu_system.run_if(in_state(GameState::Paused)),
            ),
        )
        .add_systems(
            OnEnter(GameState::Paused),
            cursor_ungrab.run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            cursor_grab.run_if(in_state(GameState::InGame)),
        )
        .insert_resource(PauseDebounceTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Once,
        )));
    }
}
