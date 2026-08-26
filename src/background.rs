use bevy::{prelude::*, window::WindowResized};
use crate::constants::{GameState, GAME_SPEED, z_index};

const GROUND_WIDTH: f32 = 1479.0; 

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct BackgroundImage;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_background)
           .add_systems(
               Update, 
               scroll_ground.run_if(in_state(GameState::Playing))
           )
           .add_systems(Update, resize_background);
    }
}

fn setup_background(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    let width = window.resolution.width();
    let height = window.resolution.height();
    let ground_height = height/10.0;
    let ground_y = -height / 2.0 + ground_height / 2.0;

    commands.spawn((
        BackgroundImage,
        Sprite {
            image: asset_server.load("bg.png"),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z_index::BACKGROUND),
    ));

    commands.spawn((
        Ground,
        Sprite {
            image: asset_server.load("ground.png"),
            custom_size: Some(Vec2::new(GROUND_WIDTH, ground_height)),
            ..default()
        },
        Transform::from_xyz(0.0, ground_y, z_index::GROUND),
    ));

    commands.spawn((
        Ground,
        Sprite {
            image: asset_server.load("ground.png"),
            custom_size: Some(Vec2::new(GROUND_WIDTH, ground_height)),
            ..default()
        },
        Transform::from_xyz(GROUND_WIDTH, ground_y, z_index::GROUND),
    ));
}

fn resize_background(
    mut window_resized: MessageReader<WindowResized>,
    mut query_bg: Query<&mut Sprite, (With<BackgroundImage>, Without<Ground>)>,
    mut query_ground: Query<(&mut Sprite, &mut Transform), With<Ground>>,
) {
    for event in window_resized.read() {
        for mut sprite in &mut query_bg {
            sprite.custom_size = Some(Vec2::new(event.width, event.height));
        }

        let ground_height = event.height / 10.0;
        let ground_y = -event.height / 2.0 + ground_height / 2.0;

        for (mut sprite, mut transform) in &mut query_ground {
            sprite.custom_size = Some(Vec2::new(GROUND_WIDTH, ground_height));
            transform.translation.y = ground_y;
        }
    }
}

fn scroll_ground(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ground>>,
) {
    for mut transform in &mut query {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        if transform.translation.x <= -GROUND_WIDTH {
            transform.translation.x += GROUND_WIDTH * 2.0;
        }
    }
}