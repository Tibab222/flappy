use bevy::prelude::*;
use rand::Rng;
use crate::{bird::Bird, constants::{GameState, PIPE_GAP, PIPE_SPAWN_TIME, PIPE_SPEED, PIPE_WIDTH, z_index}, score::{HighScore, Score}};

#[derive(Component)]
pub struct Pipe {
    pub passed: bool,
}

#[derive(Resource)]
struct PipeSpawnTimer(Timer);

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PipeSpawnTimer(Timer::from_seconds(
            PIPE_SPAWN_TIME,
            TimerMode::Repeating,
        )))
        .add_systems(OnEnter(GameState::Playing), cleanup_pipes)
        .add_systems(Update, (spawn_pipes, move_pipes, despawn_pipes, check_score).run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pipes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PipeSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        let secret_gap = rand::thread_rng().gen_range(-PIPE_GAP..=PIPE_GAP);

        let spawn_x = 400.0;
        let gap_y = secret_gap;
        let pipe_height = 400.0;
        let pipe_y_gap = (PIPE_GAP / 2.0) + (pipe_height / 2.0);

        // down pipe
        commands.spawn((
            Pipe { passed: false },
            Sprite {
                image: asset_server.load("pipe.png"),
                custom_size: Some(Vec2::new(PIPE_WIDTH, pipe_height)),
                flip_y: false,
                ..default()
            },
            Transform::from_xyz(spawn_x, gap_y - pipe_y_gap, z_index::PIPES)
        ));

        // top pipe
        commands.spawn((
            Pipe { passed: false },
            Sprite {
                image: asset_server.load("pipe.png"),
                custom_size: Some(Vec2::new(PIPE_WIDTH, pipe_height)),
                flip_y: true,
                ..default()
            },
            Transform::from_xyz(spawn_x, gap_y + pipe_y_gap, z_index::PIPES)
        ));
    }
}

fn move_pipes(time: Res<Time>, mut query: Query<&mut Transform, With<Pipe>>) {
    for mut transform in &mut query {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();
    }
}

fn despawn_pipes(mut commands: Commands, query: Query<(Entity, &Transform), With<Pipe>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_pipes(mut commands: Commands, query: Query<Entity, With<Pipe>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn check_score(
    bird_query: Query<&Transform, With<Bird>>,
    mut pipe_query: Query<(&Transform, &mut Pipe)>,
    mut score: ResMut<Score>,
    mut high_score: ResMut<HighScore>,
) {
    let Ok(bird_transform) = bird_query.single() else { return };

    for (pipe_transform, mut pipe) in &mut pipe_query {
        if !pipe.passed && bird_transform.translation.x > pipe_transform.translation.x {
            score.0 += 1;
            pipe.passed = true;

            if score.0 > high_score.0 {
                high_score.0 = score.0;
            }
        }
    }
}