use bevy::prelude::*;
use bevy::{
    prelude::{Asset, Resource},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

#[derive(Asset, TypePath, Serialize, Deserialize, Resource, Default, Clone)]
pub struct UserSettings {
    pub sensitivity: Vec2,
    pub fov: f32,
}

#[derive(Asset, TypePath, Serialize, Deserialize, Resource, Default, Clone)]
pub struct GameSettings {
    pub color_distance_factor: f32,
    pub gun_fire_rate: f32,
    pub max_shots_per_frame: u32,
    pub max_gun_spread: f32,
    pub movement_speed_factor: f32,
    pub points_limit: Option<usize>,
}
