use bevy::prelude::*;

mod bird;
mod camera;
mod constants;

use bird::BirdPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(BirdPlugin)
        .run();
}