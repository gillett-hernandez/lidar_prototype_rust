use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Resource)]
pub struct GameSettings {
    pub sensitivity: Vec2,
}
