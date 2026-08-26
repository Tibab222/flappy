use bevy::prelude::*;
use crate::constants::GameState;

#[derive(Component)]
struct MenuUI;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu_ui)
           .add_systems(OnExit(GameState::Menu), cleanup_ui)
           .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
           
           .add_systems(OnEnter(GameState::GameOver), setup_gameover_ui)
           .add_systems(OnExit(GameState::GameOver), cleanup_ui)
           .add_systems(Update, handle_gameover_input.run_if(in_state(GameState::GameOver)));
    }
}

fn setup_menu_ui(mut commands: Commands) {
    commands.spawn((
        MenuUI,
        Text::new("FLAPPY BIRD\n Press SPACE to start!"),
        Node {
            position_type: PositionType::Absolute,
            top: px(200),
            left: px(250),
            ..default()
        },
    ));
}

// UI Game Over
fn setup_gameover_ui(mut commands: Commands) {
    commands.spawn((
        MenuUI,
        Text::new("GAME OVER\n Press R to replay"),
        Node {
            position_type: PositionType::Absolute,
            top: px(200),
            left: px(270),
            ..default()
        },
    ));
}

fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn handle_menu_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
}

fn handle_gameover_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
}