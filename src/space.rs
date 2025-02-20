use std::collections::VecDeque;

use bevy::{math::vec3, picking::backend::ray::RayMap, prelude::*};

use crate::{gun::LidarShotFired, settings::GameSettings};
use crate::{material::CustomMaterial, player::Player, settings::UserSettings};

pub trait PointStorage {
    /// method to add points
    fn add_points(&mut self, points: &[Vec3], entities: &[Entity]);
    /// method that returns entities that should be despawned to adhere to some given point limit.
    fn trim(&mut self) -> Vec<Entity>;
}

pub struct VecStorage {
    pub points: VecDeque<Entity>,
    pub limit: usize,
}

impl PointStorage for VecStorage {
    fn add_points(&mut self, _: &[Vec3], entities: &[Entity]) {
        self.points.extend(entities.iter().cloned());
    }
    fn trim(&mut self) -> Vec<Entity> {
        let cur_len = self.points.len();
        if cur_len > self.limit {
            let excess_elements = cur_len - self.limit;
            self.points.drain(0..excess_elements).collect()
        } else {
            vec![]
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

    pub fn trim(&mut self) -> Vec<Entity> {
        self.accelerator.trim()
    }
}

/// tag for spheres created by the lidar shot system
#[derive(Component)]
pub struct LidarTag;

/// tag for objects that are intersectable by the lidar system
#[derive(Component)]
pub struct LidarInteractable;

// #[derive(Component)]
// pub struct ColorWrapper(Color);

#[derive(Resource, Default, Clone)]
pub struct SphereHandles {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<CustomMaterial>>,
}

// TODO: optimize more.
pub fn lidar_new_points<S: PointStorage + Send + Sync + 'static>(
    mut raycast: MeshRayCast,
    mut commands: Commands,
    mut space: ResMut<Space<S>>,
    filter_query_lidar_interactable: Query<(), With<LidarInteractable>>,
    mut new_spheres: EventReader<LidarShotFired>,
    sphere_handles: Res<SphereHandles>,
    game_settings: Res<GameSettings>,
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
    let filter = |e| filter_query_lidar_interactable.contains(e);
    let settings = RayCastSettings::default()
        .with_visibility(RayCastVisibility::Visible)
        .with_filter(&filter)
        .always_early_exit();

    for shot in new_spheres
        .read()
        .take(game_settings.max_shots_per_frame as usize)
    {
        let result = raycast
            .cast_ray(
                Ray3d::new(shot.origin, shot.direction),
                &settings,
                // &mut gizmos,
            )
            .first();
        if let Some((_entity, data)) = result {
            let entity = commands
                .spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(data.point).with_scale(Vec3::splat(light_radius)),
                ))
                .insert(LidarTag)
                // .with_children(|children| {
                //     children.spawn(PointLightBundle {
                //         point_light: PointLight {
                //             radius: light_radius,
                //             color: Color::srgb(0.2, 0.2, 1.0),
                //             ..default()
                //         },
                //         ..default()
                //     });
                // })
                .id();
            new_points.push(data.point);
            new_entities.push(entity);
        }
    }
    space.add_points(&new_points[..], &new_entities[..]);
    for entity in space.trim() {
        commands.entity(entity).despawn_recursive();
    }
}

// pub fn propagate_update_colors(
//     mut sphere_query: Query<(&mut Handle<CustomMaterial>, &GlobalTransform), With<LidarTag>>,
//     player_query: Query<&GlobalTransform, With<Player>>,
// ) {
//     let Ok(transform) = player_query.get_single() else {
//         return;
//     };

//     sphere_query.par_iter_mut().for_each(|(mat, transform)| {
//         // mat.
//     });
// }

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
