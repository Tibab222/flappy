use bevy::prelude::*;
use crate::{
    bird::Bird,
    constants::{GameState, PIPE_SPEED, z_index},
};

pub const COIN_RADIUS: f32 = 14.0;

#[derive(Component)]
pub struct Coin;

#[derive(Resource, Default)]
pub struct Wallet {
    pub coins: u32,
}

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Wallet>()
            .add_systems(OnEnter(GameState::Playing), cleanup_coins)
            .add_systems(
                Update,
                (move_coins, collect_coins, despawn_coins).run_if(in_state(GameState::Playing)),
            );
    }
}

pub fn spawn_coin(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    spawn_x: f32,
    gap_y: f32,
) {
    if rand::random_bool(0.5) {
        let coin_y = gap_y + rand::random_range(-30.0..=30.0);

        commands.spawn((
            Sprite {
                image: asset_server.load("coin.png"),
                custom_size: Some(Vec2::new(COIN_RADIUS*2.0, COIN_RADIUS*2.0)),
                ..default()
            },
            Transform::from_xyz(spawn_x, coin_y, z_index::PIPES + 0.1),
            Coin,
        ));
    }
}

fn move_coins(time: Res<Time>, mut query: Query<&mut Transform, With<Coin>>) {
    for mut transform in &mut query {
        transform.translation.x -= PIPE_SPEED * time.delta_secs();
    }
}

fn collect_coins(
    mut commands: Commands,
    bird_query: Query<&Transform, With<Bird>>,
    coin_query: Query<(Entity, &Transform), With<Coin>>,
    mut wallet: ResMut<Wallet>,
) {
    let Ok(bird_transform) = bird_query.single() else { return };
    let bird_pos = bird_transform.translation.truncate();

    for (coin_entity, coin_transform) in &coin_query {
        let coin_pos = coin_transform.translation.truncate();

        if bird_pos.distance(coin_pos) < (COIN_RADIUS + 15.0) {
            wallet.coins += 1;
            commands.entity(coin_entity).despawn();
        }
    }
}

fn despawn_coins(mut commands: Commands, query: Query<(Entity, &Transform), With<Coin>>) {
    for (entity, transform) in &query {
        if transform.translation.x < -500.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn cleanup_coins(mut commands: Commands, query: Query<Entity, With<Coin>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}