use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

pub mod art;
pub mod ui;
pub mod game;
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,

    GameSetup,
    Game,
    GameCleanup,
}

#[derive(Event)]
pub struct GameCleanupEvent {
    next_state: AppState,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_event::<GameCleanupEvent>()

        .add_plugins((
            DefaultPlugins.set(WindowPlugin { // Change window plugin to default to bordless fullscreen
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()), // Change ImagePlugin to render sprites with nearest scaling
        ))


        .add_plugins((ui::UiPlugin, game::GamePlugin))

        .add_systems(Startup, (spawn_camera, spawn_background))
        .add_systems(OnEnter(AppState::GameSetup), game_setup_transition)
        .add_systems(OnEnter(AppState::GameCleanup), game_cleanup_transition)
        .run();
}

fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let background_size = if window.width() > window.height() {
        window.width() / art::BACKGROUND_SIZE.x
    } else {
        window.height() / art::BACKGROUND_SIZE.y
    };

    commands.spawn(
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(window.width() / 2.0, window.height() / 2.0, -1.0),
                scale: Vec3::splat(background_size),
                ..default()
            },
            texture: asset_server.load(art::BACKGROUND_PATH),
            ..default()
        }
    );
}

// Spawn camera
fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        }
    );
}

// After the game is setup transition to AppState::Game
fn game_setup_transition(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::Game);
}

// After cleanup transition enter the AppState given by the event
fn game_cleanup_transition(
    mut cleanup_event: EventReader<GameCleanupEvent>,
    mut next_game_state: ResMut<NextState<game::GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    next_game_state.set(game::GameState::Running);
    for event in cleanup_event.read() {
        next_app_state.set(event.next_state);
    }
}
