extern crate sfml;
extern crate rand;
extern crate ncollide;
extern crate nalgebra as na;

use std::rc::Rc;

use sfml::graphics::{RenderWindow, Color, RenderTarget, RectangleShape, Font, Text};
use sfml::window::{VideoMode, ContextSettings, event, Close};
use sfml::window::keyboard::Key;
use sfml::system::Vector2f;

use sfml::traits::drawable::Drawable;

use ncollide::shape::{Cuboid};

use na::Vec2;

mod constants;
mod piece;

use constants::*;
mod game_state;
use game_state::{GameState, Phase};

mod assets;

fn main() {
    let assets = assets::load();
    
    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_X, WINDOW_Y, 32), "GridGame", Close, &setting)
        .expect("Cannot create a new RenderWindow");
    window.set_vertical_sync_enabled(true);

    let mut game_state = GameState::new(&assets);
    let mut curtain = RectangleShape::new().expect("Could not allocate RectangleShape!");
    curtain.set_size2f(WINDOW_X as f32, WINDOW_Y as f32);

    
    while window.is_open() {
        let start_at = game_state.game_timer();
        
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    Key::Up => game_state.move_player(Vec2::new(0., -10.)),
                    Key::Down => game_state.move_player(Vec2::new(0., 10.)),
                    Key::Left => game_state.move_player(Vec2::new(-10., 0.)),
                    Key::Right => game_state.move_player(Vec2::new(10., 0.)),
                    Key::Space | Key::Return => {
                        if game_state.phase == Phase::PlayerLost {
                            game_state.reset();
                        }
                    },
                    Key::F1 => {
                        game_state.debug_ticks = !game_state.debug_ticks;
                    },
                    Key::F2 => {
                        game_state.debug_loop = !game_state.debug_loop;
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        // Clear the window
        window.clear(&Color::black());

        // Draw the playarea
        draw_playarea(&mut window);

        match game_state.phase {
            Phase::Playing => {
                if game_state.check_tick() { game_state.tick() }

                draw_status_bar(&mut window, &game_state, &assets.f_dosis_m);
                window.draw(&game_state);
            }
            Phase::PlayerLost => {
                // Display gradient / game over based on time since loss.
                window.draw(&game_state);

                let time = game_state.ms_since_dead();
                let alpha = linear_tween(time, 0, curtain.get_fill_color().alpha, 1000);

                curtain.set_fill_color(&Color::new_rgba(0, 0, 0, alpha));

                let mut text = Text::new_init("GAME OVER", &assets.f_dosis_m, 96)
                    .expect("Failed to render text!");
                let text_rect = text.get_local_bounds();
                text.set_position2f(
                    (WINDOW_X as f32 / 2.0) - (text_rect.width / 2.0),
                    (WINDOW_Y as f32 / 2.0) - (text_rect.height / 2.0));
                text.set_color(&Color::red());
                window.draw(&text);

                window.draw(&curtain);
            },
            // Phase::LevelComplete => {},
        }
        
        // Display things on screen
        window.display();

        if game_state.debug_loop {
            println!("Main loop complete in {}ms", game_state.game_timer() - start_at);
        }
    }
}

fn draw_playarea(window: &mut RenderWindow) {
    let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape!");
    rect.set_position2f(PADDING as f32, PADDING as f32);
    rect.set_size2f(PLAYAREA_X as f32, PLAYAREA_Y as f32);
    rect.set_fill_color(&Color::new_rgb(64, 64, 64));

    window.draw(&rect);
}

fn draw_status_bar(window: &mut RenderWindow, game_state: &GameState, font: &Font) {
    let mut text = Text::new_init(
        &format!("Level: {}   Score: {}   Time: {:.2}", game_state.level, game_state.score, game_state.clock.get_elapsed_time().as_seconds()), font, 32)
        .expect("Failed to render font.");
    text.set_color(&Color::white());
    text.set_position2f(PADDING as f32, (WINDOW_Y - 32) as f32);
    window.draw(&text);
}

// t: current time
// b: start value
// c: change in value ??
// d: duration
fn linear_tween(t: i32, b: u8, c: u8, d: i32) -> u8 {
    // TODO: This is probably terribly inefficient.
    (c as f32 * (t as f32/d as f32) + b as f32) as u8
}
