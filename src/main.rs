use std::collections::HashSet;

use ::piston::input::*;
use piston_window::{graphics::clear, *};

use rust_pong::constants::{HEIGHT, WIDTH};
use rust_pong::game::Game;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Pong en Rust", [WIDTH as u32, HEIGHT as u32])
            .exit_on_esc(true)
            .resizable(false)
            .build()
            .unwrap();

    let mut game = Game::new();
    let mut pressed_keys = HashSet::new();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            game.draw(c, g);
        });

        if let Some(Button::Keyboard(key)) = e.press_args() {
            pressed_keys.insert(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            pressed_keys.remove(&key);
        }
        if let Some(_) = e.update_args() {
            game.update(&pressed_keys);
        }
    }
}
