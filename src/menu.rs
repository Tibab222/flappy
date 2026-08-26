use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};
use crate::constants::{GameState, COLOR_HOVERED_BTN, COLOR_NORMAL_BTN, COLOR_PRESSED_BTN};

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
    Menu,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu_ui)
           .add_systems(OnExit(GameState::Menu), cleanup_ui)
           .add_systems(Update, button_system.run_if(in_state(GameState::Menu)))
           
           .add_systems(OnEnter(GameState::GameOver), setup_gameover_ui)
           .add_systems(OnExit(GameState::GameOver), cleanup_ui)
           .add_systems(Update, button_system.run_if(in_state(GameState::GameOver)));
    }
}

fn setup_menu_ui(mut commands: Commands) {
    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FLAPPY BIRD"),
                TextFont::from_font_size(64.0),
                TextColor(Color::WHITE),
            ));

            spawn_button(parent, "Play", MenuButtonAction::Play);
            spawn_button(parent, "Quit", MenuButtonAction::Quit);
        });
}

// --- ECRAN GAME OVER ---
fn setup_gameover_ui(mut commands: Commands) {
    commands
        .spawn((
            MenuUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("GAME OVER"),
                TextFont::from_font_size(50.0),
                TextColor(Color::srgb(0.9, 0.2, 0.2)), // Rouge
            ));

            spawn_button(parent, "PLAY AGAIN", MenuButtonAction::Menu);
        });
}

fn spawn_button(parent: &mut RelatedSpawnerCommands<ChildOf>, text: &str, action: MenuButtonAction) {
    parent
        .spawn((
            Button,
            action,
            Node {
                width: Val::Px(200.0),
                height: Val::Px(55.0),
                border: UiRect::all(Val::Px(3.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(12.0)), // Bords arrondis
                ..default()
            },
            BackgroundColor(COLOR_NORMAL_BTN),
            BorderColor::all(Color::WHITE),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(text),
                TextFont::from_font_size(25.0),
                TextColor(Color::WHITE),
            ));
        });
}

// Click & hover
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, mut bg_color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = COLOR_PRESSED_BTN.into();
                match action {
                    MenuButtonAction::Play => next_state.set(GameState::Playing),
                    MenuButtonAction::Quit => {
                        app_exit.write(AppExit::Success);
                    },
                    MenuButtonAction::Menu => next_state.set(GameState::Menu),
                }
            }
            Interaction::Hovered => {
                *bg_color = COLOR_HOVERED_BTN.into();
            }
            Interaction::None => {
                *bg_color = COLOR_NORMAL_BTN.into();
            }
        }
    }
}

// Nettoyage de l'UI entre les transitions de states
fn cleanup_ui(mut commands: Commands, query: Query<Entity, With<MenuUI>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}