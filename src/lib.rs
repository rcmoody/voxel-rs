//! # voxel-rs
//!
//! `voxel-rs` is a voxel-based game written in Rust using wgpu.
//! This project primarily intends to serve as a learning exercise
//! in software engineering and computer graphics programming.

use std::sync::Arc;
use std::time::Instant;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowId};

use crate::consts::window;
use crate::game::Game;
use crate::renderer::Renderer;

mod consts;
mod game;
mod renderer;

/// The main application struct.
struct App<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    game: Game,
    last_time: Instant,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            window: None,
            renderer: None,
            game: Game::new(),
            last_time: Instant::now(),
        }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(
                event_loop
                    .create_window(Window::default_attributes().with_title(window::TITLE))
                    .unwrap(),
            );

            self.window = Some(Arc::clone(&window));
            self.renderer = Some(pollster::block_on(Renderer::new(Arc::clone(&window))));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.update();
                self.render();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard_input(event, event_loop);
            }
            _ => (),
        }
    }
}

impl App<'_> {
    fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now - self.last_time;
        self.last_time = now;

        self.game.update(delta_time);
    }

    fn render(&mut self) {
        let renderer = self.renderer.as_mut().unwrap();
        renderer.render(&self.game);
    }

    fn handle_keyboard_input(&mut self, key_event: KeyEvent, event_loop: &ActiveEventLoop) {
        if let KeyEvent {
            physical_key: PhysicalKey::Code(key_code),
            state: ElementState::Pressed,
            ..
        } = key_event
        {
            match key_code {
                KeyCode::Escape => event_loop.exit(),
                _ => (),
            }
            self.game.handle_keyboard_input(key_event);
        }
    }
}

/// Starts the application.
pub fn run() {
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
