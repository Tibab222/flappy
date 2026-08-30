use bevy::prelude::*;
use crate::coin::Wallet;

pub struct StoragePlugin;

impl Plugin for StoragePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_save_data);
    }
}

// --- WEB (Wasm) ---
#[cfg(target_arch = "wasm32")]
pub fn save_game_state(coins: u32) {
    use web_sys::window;

    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            let _ = storage.set_item("flappy_coins", &coins.to_string());
            web_sys::console::log_1(&"Saved on Web!".into());
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn load_save_data(mut wallet: ResMut<Wallet>) {
    use web_sys::window;

    if let Some(win) = window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(saved_coins)) = storage.get_item("flappy_coins") {
                if let Ok(coins) = saved_coins.parse::<u32>() {
                    wallet.coins = coins;
                }
            }
        }
    }
}

// --- DESKTOP later ---
#[cfg(not(target_arch = "wasm32"))]
pub fn save_game_state(coins: u32) {
    println!("Saving: {} coins", coins);
}

#[cfg(not(target_arch = "wasm32"))]
fn load_save_data(mut wallet: ResMut<Wallet>) {}