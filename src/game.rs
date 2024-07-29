use std::time::Duration;

use winit::event::KeyEvent;

/// Represents all game-related state.
pub struct Game;

impl Game {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&mut self, delta_time: Duration) {}

    pub fn handle_keyboard_input(&mut self, key_event: KeyEvent) {}
}
