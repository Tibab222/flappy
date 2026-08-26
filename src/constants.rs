use bevy::prelude::*;

pub mod z_index {
    pub const BACKGROUND: f32 = -10.0;
    pub const PIPES: f32 = 1.0;
    pub const BIRD: f32 = 0.0;
    // pub const UI: f32 = 10.0;
    pub const CAMERA: f32 = 1.0;
    pub const GROUND: f32 = 1.0;
}

// bird
pub const GRAVITY: f32 = 900.0;
pub const JUMP_IMPULSE: f32 = 350.0;

// Pipes
pub const PIPE_SPEED: f32 = 150.0;
pub const GAME_SPEED: f32 = 150.0;
pub const PIPE_SPAWN_TIME: f32 = 1.5; // one pipe per 1.5s
pub const PIPE_GAP: f32 = 120.0; // between two vertical pipes
pub const PIPE_WIDTH: f32 = 50.0;

// states
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
}

// Colors
pub const COLOR_NORMAL_BTN: Color = Color::srgb(0.2, 0.6, 0.86);
pub const COLOR_HOVERED_BTN: Color = Color::srgb(0.3, 0.73, 1.0);
pub const COLOR_PRESSED_BTN: Color = Color::srgb(0.12, 0.45, 0.67);