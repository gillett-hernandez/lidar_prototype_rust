use std::collections::VecDeque;

use bevy_mod_raycast::prelude::*;

use bevy::prelude::*;

use crate::{gun::LidarShotFired, player::Player, settings::GameSettings};

pub trait PointStorage {
    fn add_points(&mut self, points: &[Vec3], entities: &[Entity]);
}

pub struct VecStorage {
    pub points: VecDeque<(Vec3, Entity)>,
    pub limit: usize,
}

impl PointStorage for VecStorage {
    /// drops excess points if too many points are added.
    fn add_points(&mut self, points: &[Vec3], entities: &[Entity]) {
        self.points
            .extend(points.iter().cloned().zip(entities.iter().cloned()));
        let cur_len = self.points.len();
        if cur_len > self.limit {
            let excess_elements = cur_len - self.limit;
            self.points.drain(0..excess_elements);
        }
    }
}

#[derive(Resource)]
pub struct Space<S: PointStorage> {
    pub accelerator: S,
}

impl<S: PointStorage> Space<S> {
    pub fn add_points(&mut self, points: &[Vec3], entities: &[Entity]) {
        self.accelerator.add_points(points, entities);
    }
}

#[derive(Component)]
pub struct LidarTag;

#[derive(Component)]
pub struct ColorWrapper(Color);

#[derive(Resource, Default, Clone)]
pub struct SphereHandles {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

pub fn lidar_new_points<S: PointStorage + Send + Sync + 'static>(
    mut raycast: Raycast,
    mut gizmos: Gizmos,
    mut commands: Commands,
    mut space: ResMut<Space<S>>,
    mut new_spheres: EventReader<LidarShotFired>,
    sphere_handles: Res<SphereHandles>,
) {
    let Some(ref mesh) = sphere_handles.mesh else {
        return;
    };
    let Some(ref material) = sphere_handles.material else {
        return;
    };

    let light_radius = 1.0;

    let mut new_points = Vec::new();
    let mut new_entities = Vec::new();
    for shot in new_spheres.read() {
        let result = raycast
            .debug_cast_ray(
                Ray3d::new(shot.origin, *shot.direction),
                &default(),
                &mut gizmos,
            )
            .first();
        if let Some((_entity, data)) = result {
            let entity = commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    transform: Transform::from_translation(data.position())
                        .with_scale(Vec3::splat(light_radius)),
                    ..default()
                })
                .insert(LidarTag)
                .with_children(|children| {
                    children.spawn(PointLightBundle {
                        point_light: PointLight {
                            radius: light_radius,
                            color: Color::srgb(0.2, 0.2, 1.0),
                            ..default()
                        },
                        ..default()
                    });
                })
                .id();
            new_points.push(data.position());
            new_entities.push(entity);
        }
    }
    space.add_points(&new_points[..], &new_entities[..]);
}

// pub fn propagate_update_colors(parent_query: Query<(Entity, &Children), With<LidarTag>>) {}

// pub fn lidar_sphere_render_manager<S: PointStorage + Send + Sync + 'static>(
//     space: Res<Space<S>>,
//     points: Query<(&mut ColorWrapper, &GlobalTransform), With<LidarTag>>,
//     player_change: Query<(&GlobalTransform, &Player), Changed<GlobalTransform>>,
//     game_settings: Res<GameSettings>,
// ) {
//     let color_distance_scale = game_settings.color_distance_scale;
//     if let Ok((transform, player)) = player_change.get_single() {
//         // using new player location, update all the spheres
//     }
// }
