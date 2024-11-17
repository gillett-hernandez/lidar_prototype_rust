use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::{input::PlayerInput, space::Space};
use rand;

#[derive(Event, Copy, Clone, Debug)]
pub struct LidarShotFired {
    origin: Vec3,
    direction: Dir3,
}

#[derive(Component)]

pub struct LidarGun {
    // radians from center of the aim direction to the maximal extent of the spread
    pub current_angular_spread_radius: f32,
    pub fire_rate_per_second: f32,
    // TODO: add max_fire_rate and a better charge up mechanic.
    // currently,
    // pub max_fire_rate: f32,
    saved_time_secs: f32,
}

impl LidarGun {
    pub fn charge(&mut self, time: f32) {
        self.saved_time_secs += time;
    }

    pub fn shoot(&mut self) -> usize {
        let mut num = (self.saved_time_secs * self.fire_rate_per_second).floor();

        self.saved_time_secs -= num / self.fire_rate_per_second;
        self.saved_time_secs = self.saved_time_secs.max(0.0);
        num as usize
    }
}

pub fn lidar_basic_shot_system(
    mut query: Query<(&mut LidarGun, &Transform)>,
    time: Res<Time>,
    player_input: Res<PlayerInput>,
    mut shots: EventWriter<LidarShotFired>,
) {
    let delta = time.delta_seconds();

    for (mut lidar_data, transform) in query.get_single_mut() {
        lidar_data.charge(delta);
        let origin = transform.translation;
        // could use base_direction, left, and up instead of compute_matrix and transform_vector3
        // let base_direction = transform.forward();

        let cos = lidar_data.current_angular_spread_radius.cos();
        for _ in 0..lidar_data.shoot() {
            // sample and send event
            let u: f32 = rand::random();
            let v: f32 = rand::random();
            let (mut y, mut x) = (TAU * u).sin_cos();
            let z: f32 = 1.0 + v * (cos - 1.0);
            let r = (1.0 - z.powi(2)).sqrt();
            x *= r;
            y *= r;
            let dir = Vec3::new(x, y, z);
            shots.send(LidarShotFired {
                origin: origin,
                direction: Dir3::new_unchecked(transform.compute_matrix().transform_vector3(dir)),
            });
        }
    }
}
