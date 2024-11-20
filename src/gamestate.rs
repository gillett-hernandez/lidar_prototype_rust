use bevy::prelude::*;

// use bevy::time::Timer;

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug, Hash, States, Default)]
pub enum GameState {
    #[default]
    Loading, // can transition to mainmenu
    MainMenu,   // can transition to inhanger or ingame (quickstart)
    InGame,     // can transition to game ending and hitstun
    Paused,     // can transition to game ending and quitting
    GameEnding, // can transition to mainmenu or inhanger
    Quitting,   // quits the game, saving player data to disk and despawning all entities
}

#[derive(Resource, DerefMut, Deref)]
pub struct GameEndingTimer(pub Timer);

pub fn game_ending_system<C: Component>(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<GameEndingTimer>,
    mut game_state: ResMut<NextState<GameState>>,
    entity_query: Query<Entity, With<C>>,
) {
    timer.tick(time.delta());
    for entity in &entity_query {
        commands.entity(entity).despawn_recursive();
    }

    if timer.finished() {
        game_state.set(GameState::MainMenu);
        timer.reset();
    }
}
