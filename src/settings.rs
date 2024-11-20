use bevy::prelude::*;
use bevy::{
    prelude::{Asset, Resource},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Serialize, Deserialize, Resource, Default, Clone)]
pub struct GameSettings {
    pub sensitivity: Vec2,
    pub color_distance_scale: f32,
}
