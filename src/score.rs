use bevy::prelude::*;
use crate::constants::GameState;

#[derive(Resource, Default)]
pub struct Score(pub u32);

#[derive(Resource, Default)]
pub struct HighScore(pub u32);

#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
           .init_resource::<HighScore>()
           .add_systems(Startup, setup_score_ui)
           .add_systems(Update, update_score_ui)
           .add_systems(OnEnter(GameState::Playing), reset_score);
    }
}

fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

fn setup_score_ui(mut commands: Commands) {
    commands.spawn((
        ScoreText,
        Text::new("0"),
        TextFont::from_font_size(60.0),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Percent(50.0),
            margin: UiRect::left(Val::Px(-30.0)), 
            ..default()
        },
    ));
}

fn update_score_ui(
    score: Res<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    if score.is_changed() {
        for mut text in &mut query {
            **text = format!("{}", score.0);
        }
    }
}