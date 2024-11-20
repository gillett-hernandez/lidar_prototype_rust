use bevy::prelude::*;
use rand;
use std::f32::consts::TAU;

/// samples a direction within a specific solid angle, specified by the angular radius.
/// centered around Vec3::Z
pub fn solid_angle_sample(angular_radius_radians: f32) -> Vec3 {
    let cos = angular_radius_radians.cos();
    let u: f32 = rand::random();
    let v: f32 = rand::random();
    let (mut y, mut x) = (TAU * u).sin_cos();
    let z: f32 = 1.0 + v * (cos - 1.0);
    let r = (1.0 - z.powi(2)).sqrt();
    x *= r;
    y *= r;
    Vec3::new(x, y, z)
}
