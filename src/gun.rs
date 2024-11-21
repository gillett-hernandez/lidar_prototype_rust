use bevy::prelude::*;

use crate::{
    input::{FiringMode, PlayerInput},
    space::Space,
    util::solid_angle_sample,
};

#[derive(Event, Copy, Clone, Debug)]
pub struct LidarShotFired {
    pub origin: Vec3,
    pub direction: Dir3,
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
        let num = (self.saved_time_secs * self.fire_rate_per_second).floor();

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
    match player_input.firing_mode {
        FiringMode::None => {}
        FiringMode::Firing => {
            let delta = time.delta_seconds();

            let Ok((mut lidar_data, transform)) = query.get_single_mut() else {
                return;
            };
            info!("lidar basic shot system: charging up");
            lidar_data.charge(delta);
            let origin = transform.translation;
            // could use base_direction, left, and up instead of compute_matrix and transform_vector3
            // let base_direction = transform.forward();

            for _ in 0..lidar_data.shoot() {
                // sample and send event
                let dir = solid_angle_sample(lidar_data.current_angular_spread_radius);
                shots.send(LidarShotFired {
                    origin,
                    direction: Dir3::new_unchecked(
                        transform.compute_matrix().transform_vector3(dir),
                    ),
                });
            }
        }
        FiringMode::Burst => todo!(),
    }
}
