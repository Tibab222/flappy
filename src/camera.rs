use bevy::prelude::*;
use crate::constants::z_index;

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    println!("Setting up camera");
    commands.spawn((
        MainCamera,
        Transform::from_xyz(0.0, 0.0, z_index::CAMERA),
    ));
}