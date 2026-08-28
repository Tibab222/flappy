use bevy::prelude::*;

mod bird;
mod camera;
mod constants;
mod pipe;
mod collisions;
mod menu;
mod background;
mod score;
mod coin;

use bird::BirdPlugin;
use camera::CameraPlugin;
use pipe::PipePlugin;
use collisions::CollisionsPlugin;
use constants::GameState;
use menu::MenuPlugin;
use background::BackgroundPlugin;
use score::ScorePlugin;
use coin::CoinPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_plugins(CameraPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(PipePlugin)
        .add_plugins(CollisionsPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(BackgroundPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(CoinPlugin)
        .run();
}