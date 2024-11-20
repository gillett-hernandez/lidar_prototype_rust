use std::path::Path;

use bevy::{
    asset::RecursiveDependencyLoadState,
    prelude::*,
    scene::ron::{ser::PrettyConfig, Serializer},
};
use serde::Serialize;
// use bevy_kira_audio::AudioSource;

use crate::{gamestate::GameState, settings::GameSettings};

#[derive(Resource, Deref)]
pub struct AssetsTracking(pub Vec<UntypedHandle>);
impl AssetsTracking {
    pub fn new() -> Self {
        AssetsTracking(vec![])
    }
    pub fn add(&mut self, handle: UntypedHandle) {
        self.0.push(handle);
    }
}

const USER_CONFIG_FILE: &'static str = "user.rconfig";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsTracking>,
) {
    // pngs
    // for image_path in [
    //     "images/background.png",
    //     "images/player.png",
    //     "images/bullet.png",
    //     "images/enemy/basic_enemy.png",
    // ] {
    //     let handle: Handle<Image> = asset_server.load(image_path);
    //     loading.add(handle.untyped());
    // }

    for audio_path in ["sfx/hit_sound.ogg"] {
        let handle: Handle<AudioSource> = asset_server.load(audio_path);
        loading.add(handle.clone().untyped());
        commands.spawn(AudioBundle {
            source: handle,
            settings: PlaybackSettings {
                volume: bevy::audio::Volume::new(0.1),
                paused: true,
                spatial: true,
                ..default()
            },
        });
    }

    let path = Path::new("assets").join(USER_CONFIG_FILE);
    if let Ok(file) = std::fs::File::create_new(path) {
        // will error if the file already exists
        let mut serializer = Serializer::new(file, Some(PrettyConfig::new().depth_limit(4)))
            .expect("couldn't create serializer");
        let result = GameSettings::default().serialize(&mut serializer);
        result.expect("could not write to file");
    }
    let handle: Handle<GameSettings> = asset_server.load(USER_CONFIG_FILE);
    loading.add(handle.untyped());
    info!("loading {} items", loading.0.len());
}

pub fn loading_state_watcher<T: Asset>(
    mut loads: EventReader<AssetEvent<T>>,
    // server: Res<AssetServer>,
    // loading: Res<AssetsTracking>,
) {
    for load in loads.read() {
        match load {
            AssetEvent::Added { id } => {
                info!("asset {} added", id.to_string());
            }
            AssetEvent::Modified { id } => {
                info!("asset {} modified", id.to_string());
            }
            AssetEvent::Removed { id } => {
                info!("asset {} removed", id.to_string());
            }
            AssetEvent::LoadedWithDependencies { id } => {
                info!("asset {} loaded with deps", id.to_string());
            }
            AssetEvent::Unused { id } => {}
        }
    }
}

pub fn loading_update(
    mut game_config: ResMut<GameSettings>,
    mut state: ResMut<NextState<GameState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsTracking>,
    game_config_asset: Res<Assets<GameSettings>>,
) {
    // splash screen, loading progress, and transition to main menu

    // TODO: splash screen

    let mut all_done = true;
    for handle in loading.iter() {
        match server.get_load_states(handle.id()).map(|tuple| tuple.2) {
            Some(RecursiveDependencyLoadState::Loaded) => {}
            Some(RecursiveDependencyLoadState::Failed) => {
                let handle_path = handle.path();
                error!("asset failed to load, {} - {:?}", handle.id().to_string(), handle_path);
            }
            _ => {
                all_done = false;
            }
        }
    }
    if all_done {
        *game_config = game_config_asset
            .get(
                server
                    .get_handle(USER_CONFIG_FILE)
                    .expect("didn't find config file handle in asset server")
                    .id(),
            )
            .expect("didn't find config struct in asset server")
            .clone();

        state.set(GameState::MainMenu);
    }
}
