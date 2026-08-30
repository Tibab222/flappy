use bevy::prelude::*;
use crate::bird::Bird;
use crate::coin::Wallet;
use crate::pipe::Pipe;
use crate::constants::{GameState, PIPE_WIDTH};
use crate::shop::ActiveBoosters;
use crate::storage::save_game_state;

const BIRD_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const PIPE_SIZE: Vec2 = Vec2::new(PIPE_WIDTH, 400.0);

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collisions.run_if(in_state(GameState::Playing)));
    }
}

fn check_collisions(
    bird_query: Query<&Transform, With<Bird>>,
    pipe_query: Query<(Entity, &Transform), With<Pipe>>,
    mut boosters: ResMut<ActiveBoosters>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    wallet: Res<Wallet>,
) {
    if let Ok(bird_transform) = bird_query.single() {
        let bird_pos = bird_transform.translation.truncate();

        for (pipe_entity, pipe_transform) in &pipe_query {
            let pipe_pos = pipe_transform.translation.truncate();

            let dx = (bird_pos.x - pipe_pos.x).abs();
            let dy = (bird_pos.y - pipe_pos.y).abs();

            let overlap_x = dx < (BIRD_SIZE.x + PIPE_SIZE.x) / 2.0;
            let overlap_y = dy < (BIRD_SIZE.y + PIPE_SIZE.y) / 2.0;

            if overlap_x && overlap_y {
                // boosters
                if boosters.shield_charges > 0 {
                    boosters.shield_charges -= 1;
                    commands.entity(pipe_entity).despawn();
                    break;
                } else {
                    handle_game_over(&mut next_state, &wallet);
                    break;
                }
            }
        }

        if bird_pos.y < -300.0 || bird_pos.y > 300.0 {
            handle_game_over(&mut next_state, &wallet);
        }
    }
}

fn handle_game_over(
    next_state: &mut ResMut<NextState<GameState>>,
    wallet: &Res<Wallet>,
) {
    save_game_state(wallet.coins);
    next_state.set(GameState::GameOver);
}