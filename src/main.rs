use std::time::Duration;

use bevy::core_pipeline::bloom::Bloom;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy::render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat},
};

use bevy_common_assets::ron::RonAssetPlugin;
use iyes_perf_ui::prelude::PerfUiDefaultEntries;
use iyes_perf_ui::PerfUiPlugin;

pub mod assets;
pub mod gamestate;
pub mod gun;
pub mod input;
pub mod material;
pub mod pause;
pub mod player;
pub mod settings;
pub mod space;
pub mod util;

use assets::{load_assets, loading_state_watcher, loading_update, AssetsTracking};
use gamestate::{game_ending_system, GameEndingTimer, GameState};
use gun::{lidar_basic_shot_system, lidar_spread_sync, LidarGun, LidarShotFired};
use input::{player_firing_sync, player_input_system, PlayerInput};
use material::CustomMaterial;
use pause::PausePlugin;
use player::{player_movement_system, Player};
use settings::{GameSettings, UserSettings};
use space::{lidar_new_points, LidarInteractable, LidarTag, Space, SphereHandles, VecStorage};

#[derive(Resource, DerefMut, Deref)]
pub struct DebugTimer(Timer);

fn debug_timer_ticker(time: Res<Time>, mut timer: ResMut<DebugTimer>) {
    timer.tick(time.delta());
}

fn observe_game_state(space: Res<Space<VecStorage>>, debug_timer: Res<DebugTimer>) {
    if debug_timer.just_finished() {
        // dbg!(state.get());
        dbg!(space.accelerator.points.len());
    }
}

fn setup_meshes(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut sphere_handles: ResMut<SphereHandles>,
) {
    let shape = meshes.add(
        Sphere::default()
            .mesh()
            .ico(1)
            .unwrap()
            .scaled_by(Vec3::new(0.1, 0.1, 0.1)),
    );

    let material = materials.add(CustomMaterial::default());
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

fn setup_player(
    mut commands: Commands,
    user_settings: Res<UserSettings>,
    game_settings: Res<GameSettings>,
) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Visible,
        ))
        .insert(LidarGun::new(0.4, game_settings.gun_fire_rate))
        .with_children(|e| {
            e.spawn((
                Camera {
                    hdr: true,
                    ..default()
                },
                Camera3d { ..default() },
                Projection::Perspective(PerspectiveProjection {
                    fov: user_settings.fov.clamp(45.0, 110.0).to_radians(),
                    ..default()
                }),
                Tonemapping::TonyMcMapface,
                Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::X, Vec3::Y),
                Bloom::NATURAL,
            ));
        });
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>, // textures
    mut materials: ResMut<Assets<StandardMaterial>>,
    ass: Res<AssetServer>,
) {
    commands.spawn(PerfUiDefaultEntries::default());

    // note that we have to include the `Scene0` label
    // let scene_gltf = ass.load("main.glb#Scene0");

    // if we swap to using scenes, they'll need to be preprocessed somehow to get all the materials to be completely Transparent with AlphaBlend set to Add
    // maybe also somehow pre-adding the LidarInteractable component
    // commands
    //     .spawn(SceneBundle {
    //         scene: scene_gltf,
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..Default::default()
    //     })
    //     .insert(LidarInteractable);

    let completely_transparent_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.0, 0.0, 0.0, 0.0),
        alpha_mode: AlphaMode::Add,
        ..default()
    });

    let shape = meshes.add(Cuboid::new(5.0, 2.0, 5.0));
    let actual_material = completely_transparent_material;

    commands
        .spawn((
            Mesh3d(shape),
            MeshMaterial3d(actual_material.clone()),
            Visibility::Visible,
            Transform::from_xyz(0.0, 2.0, 0.0),
        ))
        .insert(LidarInteractable);

    let plane = meshes.add(
        Plane3d::default()
            .mesh()
            .size(50.0, 50.0)
            .subdivisions(5)
            .build(),
    );

    commands
        .spawn((
            Mesh3d(plane),
            MeshMaterial3d(actual_material.clone()),
            Visibility::Visible,
            Transform::from_xyz(0.0, 2.0, 0.0),
        ))
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
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        // debug resources and systems
        .insert_resource(DebugTimer(Timer::new(
            Duration::from_millis(500),
            TimerMode::Repeating,
        ))) // debug timer
        .add_systems(Update, debug_timer_ticker)
        .add_systems(Update, observe_game_state)
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(PerfUiPlugin)
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
                limit: 80000, // TODO: have this alterable from game_config
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
            PreUpdate,
            player_input_system.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            (
                lidar_new_points::<VecStorage>,
                player_movement_system,
                lidar_basic_shot_system,
                player_firing_sync,
                lidar_spread_sync,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            game_ending_system::<LidarTag>.run_if(in_state(GameState::GameEnding)),
        );

    app.run();
}
