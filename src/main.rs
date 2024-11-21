use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::{
    color::palettes::basic::SILVER,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use std::time::Duration;

pub mod assets;
pub mod gamestate;
pub mod gun;
pub mod input;
pub mod pause;
pub mod player;
pub mod settings;
pub mod space;
pub mod util;

use assets::{load_assets, loading_state_watcher, loading_update, AssetsTracking};
use bevy_common_assets::ron::RonAssetPlugin;
use gamestate::{game_ending_system, GameEndingTimer, GameState};
use gun::{lidar_basic_shot_system, LidarGun, LidarShotFired};
use input::{player_firing_sync, player_input_system, PlayerInput};
use pause::PausePlugin;
use player::{player_movement_system, PlayerBundle};
use settings::{GameSettings, UserSettings};
use space::{lidar_new_points, LidarInteractable, LidarTag, Space, SphereHandles, VecStorage};

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

fn setup_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut sphere_handles: ResMut<SphereHandles>,
) {
    let shape = meshes.add(
        Sphere::default()
            .mesh()
            .ico(3)
            .unwrap()
            .scaled_by(Vec3::new(0.1, 0.1, 0.1)),
    );

    let material = materials.add(StandardMaterial {
        emissive: LinearRgba::rgb(3.0, 3.0, 3.0),
        ..default()
    });
    sphere_handles.mesh = Some(shape);
    sphere_handles.material = Some(material);
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

fn setup_player(mut commands: Commands) {
    commands
        .spawn(PlayerBundle::new(SpatialBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.0),
        )))
        .insert(LidarGun::new(0.01, 1000.0))
        .with_children(|e| {
            e.spawn((
                Camera3dBundle {
                    camera: Camera {
                        hdr: true, // 1. HDR is required for bloom
                        ..default()
                    },
                    tonemapping: Tonemapping::TonyMcMapface,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::X, Vec3::Y),

                    ..default()
                },
                BloomSettings::NATURAL,
            ));
        });
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

    commands
        .spawn(PbrBundle {
            mesh: shape,
            material: actual_material.clone(),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(LidarInteractable);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10)),
            material: materials.add(Color::from(SILVER)),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(LidarInteractable);
}

pub fn dummy_mainmenu(
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    if matches!(current_state.get(), GameState::MainMenu) {
        next_state.set(GameState::InGame);
    }
}

const USERFILE_EXTENSION: &[&'static str] = &["ron"];
const CONFIG_FILE_EXTENSION: &[&'static str] = &["rconfig"];

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
        .add_plugins(RonAssetPlugin::<UserSettings>::new(USERFILE_EXTENSION))
        .add_plugins(RonAssetPlugin::<GameSettings>::new(CONFIG_FILE_EXTENSION))
        // misc plugins
        .add_plugins(PausePlugin)
        // misc events and resources
        .add_event::<LidarShotFired>()
        .insert_resource(PlayerInput::default())
        .insert_resource(SphereHandles::default())
        .insert_resource(UserSettings::default())
        .insert_resource(GameSettings::default())
        .insert_resource(GameEndingTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Once,
        )))
        .insert_resource(Space {
            accelerator: VecStorage {
                points: vec![].into(),
                limit: 1000000, // TODO: have this alterable from game_config
            },
        })
        // systems
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            Update,
            (
                loading_update,
                loading_state_watcher::<UserSettings>,
                // loading_state_watcher::<Image>,
            )
                .run_if(in_state(GameState::Loading)),
        )
        .add_systems(Startup, setup_meshes)
        .add_systems(
            OnTransition {
                exited: GameState::MainMenu,
                entered: GameState::InGame,
            },
            (setup_player, setup_scene),
        )
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
