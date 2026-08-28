use bevy::input::touch::TouchPhase;
use bevy::prelude::*;
use crate::constants::GameState;
use crate::constants::z_index;
use crate::constants::GRAVITY;
use crate::constants::JUMP_IMPULSE;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Velocity(pub f32); // speed on y axis

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bird);
        app.add_systems(OnEnter(GameState::Playing), reset_bird)
            .add_systems(Update, (apply_gravity, bird_jump, bird_orientation).run_if(in_state(GameState::Playing)));
    }
}

// systems
fn spawn_bird(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning bird");
    commands.spawn((
        Bird, 
        Velocity(0.0),
        Sprite {
            image: asset_server.load("yellow_bird.png"),
            custom_size: Some(Vec2::new(34.0, 24.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z_index::BIRD),
    ));
}

fn reset_bird(mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>) {
    if let Ok((mut transform, mut velocity)) = query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, z_index::BIRD);
        transform.rotation = Quat::IDENTITY; // Remet la rotation à 0
        velocity.0 = 0.0;
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Bird>>
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        velocity.0 -= GRAVITY * time.delta_secs();
        transform.translation.y += velocity.0 * time.delta_secs();
    }
}

fn bird_jump(
    keyboard: Res<ButtonInput<KeyCode>>, 
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    touch_input: Res<Touches>,
    mut query: Query<&mut Velocity, With<Bird>>
) {
    let just_pressed = keyboard.just_pressed(KeyCode::Space)
        || mouse_button_input.just_pressed(MouseButton::Left)
        || touch_input.any_just_pressed();
    
    if just_pressed {
        for mut velocity in &mut query {
            velocity.0 = JUMP_IMPULSE;
        }
    }
}

fn bird_orientation(mut query: Query<(&Velocity, &mut Transform), With<Bird>>) {
    for (velocity, mut transform) in &mut query {
        let target_angle = (velocity.0 / 400.0).clamp(-1.0, 0.5);
        transform.rotation = Quat::from_rotation_z(target_angle);
    }
}