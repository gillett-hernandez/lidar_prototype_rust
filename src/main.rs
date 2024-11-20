use bevy::prelude::*;
use bevy::{
    color::palettes::basic::SILVER,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use std::time::Duration;

use assets::{load_assets, loading_state_watcher, loading_update, AssetsTracking};
use bevy_common_assets::ron::RonAssetPlugin;
use gamestate::{game_ending_system, GameEndingTimer, GameState};
use gun::{lidar_basic_shot_system, LidarShotFired};
use input::{player_firing_sync, player_input_system, PlayerInput};
use player::{player_movement_system, PlayerBundle};
use settings::GameSettings;
use space::{lidar_new_points, LidarTag, Space, SphereHandles, VecStorage};

pub mod assets;
pub mod gamestate;
pub mod gun;
pub mod input;
pub mod player;
pub mod settings;
pub mod space;
pub mod util;

#[derive(Resource, DerefMut, Deref)]
pub struct DebugTimer(Timer);

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<DebugTimer>) {
    timer.tick(time.delta());
}

fn observe_game_state(state: Res<State<GameState>>, debug_timer: Res<DebugTimer>) {
    if debug_timer.just_finished() {
        dbg!(state.get());
    }
}

const EXTENSIONS: &[&'static str] = &["rconfig"];

fn setup_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sphere_handles: ResMut<SphereHandles>,
) {
    let shape = meshes.add(Sphere::default().mesh().ico(5).unwrap());

    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 1.0),
        unlit: true,
        ..default()
    });
    sphere_handles.mesh = Some(shape);
    sphere_handles.material = Some(material);
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn(PlayerBundle::new(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0),
        )))
        .with_children(|e| {
            e.spawn(Camera3dBundle {
                transform: Transform::from_xyz(10.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
        });
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>, // textures
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let completely_transparent_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        ..default()
    });

    let shape = meshes.add(Cuboid::new(5.0, 2.0, 5.0));
    // let actual_material = completely_transparent_material;
    let actual_material = debug_material;

    commands.spawn(PbrBundle {
        mesh: shape,
        material: actual_material.clone(),
        transform: Transform::from_xyz(0.0, 2.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });
}

pub fn dummy_mainmenu(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if matches!(current_state.get(), GameState::MainMenu) {
        next_state.set(GameState::InGame);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // debug resources and systems
        .insert_resource(DebugTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        ))) // debug timer
        .add_systems(Update, debug_timer_ticker)
        .add_systems(Update, observe_game_state)
        // game state
        .insert_state::<GameState>(GameState::Loading)
        // assets
        .insert_resource(AssetsTracking::new())
        .add_plugins(RonAssetPlugin::<GameSettings>::new(EXTENSIONS))
        // misc events and resources
        .add_event::<LidarShotFired>()
        .insert_resource(PlayerInput::default())
        .insert_resource(SphereHandles::default())
        .insert_resource(GameSettings::default())
        .insert_resource(GameEndingTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .insert_resource(Space {
            accelerator: VecStorage {
                points: vec![].into(),
                limit: 10000,
            },
        })
        // systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            (
                loading_update,
                loading_state_watcher::<GameSettings>,
                // loading_state_watcher::<Image>,
            )
                .run_if(in_state(GameState::Loading)),
        )
        .add_systems(Startup, setup_meshes)
        .add_systems(OnEnter(GameState::InGame), (setup_player, setup_scene))
        .add_systems(
            Update,
            (dummy_mainmenu).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                lidar_new_points::<VecStorage>,
                player_movement_system,
                player_input_system,
                lidar_basic_shot_system,
                player_movement_system,
                player_firing_sync,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            game_ending_system::<LidarTag>.run_if(in_state(GameState::GameEnding)),
        )
        .run();
}