use bevy::prelude::*;

use crate::{
    input::{FiringMode, PlayerInput},
    settings::GameSettings,
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
    pub fn new(angular_spread: f32, fire_rate: f32) -> Self {
        Self {
            current_angular_spread_radius: angular_spread,
            fire_rate_per_second: fire_rate,
            saved_time_secs: 0.0,
        }
    }
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
    match &player_input.firing_mode {
        FiringMode::None => {}
        FiringMode::Firing => {
            let delta = time.delta_seconds();

            let Ok((mut lidar_data, transform)) = query.get_single_mut() else {
                return;
            };
            lidar_data.charge(delta);
            let origin = transform.translation;
            // could use base_direction, left, and up instead of compute_matrix and transform_vector3
            // let base_direction = transform.forward();

            for _ in 0..lidar_data.shoot() {
                // sample and send event
                let dir = solid_angle_sample(lidar_data.current_angular_spread_radius);
                shots.send(LidarShotFired {
                    origin,
                    direction: Dir3::new(transform.compute_matrix().transform_vector3(dir.zxy()))
                        .expect("failed to construct direction from sample, should not happen"),
                });
            }
        }
        FiringMode::Burst(_) => todo!(),
    }
}

pub fn lidar_spread_sync(
    mut query: Query<&mut LidarGun>,
    player_input: Res<PlayerInput>,
    settings: Res<GameSettings>,
) {
    let Ok(mut lidar_data) = query.get_single_mut() else {
        return;
    };

    if lidar_data.current_angular_spread_radius == 0.0 && player_input.gun_spread_intent > 0.0 {
        // collapsed to 0, need to fix
        lidar_data.current_angular_spread_radius = 0.001;
    } else {
        lidar_data.current_angular_spread_radius *= 1.01f32.powf(player_input.gun_spread_intent);

        if lidar_data.current_angular_spread_radius > settings.max_gun_spread {
            lidar_data.current_angular_spread_radius = settings.max_gun_spread;
        }
    }
}
