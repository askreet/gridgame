//! Example from SFML: Shape

extern crate sfml;
extern crate rand;

use sfml::graphics::{RenderWindow, Color, Shape, RenderTarget, Vertex, VertexArray, PrimitiveType, RectangleShape, Texture, Sprite, Font, Text};
use sfml::window::{VideoMode, ContextSettings, event, Close};
use sfml::window::keyboard::Key;
use sfml::system::Clock;
use sfml::system::Vector2f;

use std::thread;

mod constants;
use constants::*;
mod piece;
use piece::{Piece};
mod game_state;
use game_state::{Entity, GameState, Phase};


fn main() {
    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_X, WINDOW_Y, 32), "GridGame", Close, &setting)
        .expect("Cannot create a new RenderWindow");
    window.set_vertical_sync_enabled(true);

    // Load textures
    let player_texture = Texture::new_from_file("data/player-scaled.png")
        .expect("Cannot load player-scaled.png!");
    let enemy_texture = Texture::new_from_file("data/enemy.png")
        .expect("Cannot load enemy.png!");
    let treasure_texture = Texture::new_from_file("data/treasure.png")
        .expect("Cannot load treasure.png");

    // Load Fonts
    let dosis_medium_font = Font::new_from_file("data/Dosis/Dosis-Medium.ttf")
        .expect("Could not load Dosis-Medium.ttf!");

    let mut game_state = GameState::new(&player_texture, &enemy_texture, &treasure_texture);
    let mut last_enemy_movement: f32 = 0.0;
    
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    Key::Up => game_state.move_player(0, -1),
                    Key::Down => game_state.move_player(0, 1),
                    Key::Left => game_state.move_player(-1, 0),
                    Key::Right => game_state.move_player(1, 0),
                    _ => {}
                },
                // TODO: WAT
                // event::Resized => {},
                _ => {}
            };
        }
        // Clear the window
        window.clear(&Color::black());
        draw_grid(&mut window);

        match game_state.phase {
            Phase::Playing => {
                if last_enemy_movement < game_state.clock.get_elapsed_time().as_seconds() - ENEMY_MOVE_FREQ {
                    last_enemy_movement = game_state.clock.get_elapsed_time().as_seconds();
                    game_state.move_enemies();
                }

                draw_status_bar(&mut window, &game_state, &dosis_medium_font);
                game_state.draw_all(&mut window);
            }
            Phase::PlayerLost => {
                // Display gradient / game over based on time since loss.
                game_state.draw_all(&mut window);
                let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape!");
                rect.set_size2f(WINDOW_X as f32, WINDOW_Y as f32);

                let time = game_state.seconds_since_dead();
                let alpha: u8 = if time >= 1.0 {
                    190
                } else {
                    ((time / 1.0) * 190.0).floor() as u8
                };

                rect.set_fill_color(&Color::new_rgba(0, 0, 0, alpha));
                let mut text = Text::new_init("GAME OVER", &dosis_medium_font, SQUARE_SIZE as u32 * 2)
                    .expect("Failed to render text!");
                let mut textRect = text.get_local_bounds();
                println!("textRect.width = {}, / 2.0 = {}", textRect.height, textRect.height / 2.0);
                println!("math says {}", WINDOW_X as f32 - (textRect.width / 2.0));
                text.set_position2f(
                    (WINDOW_X as f32 / 2.0) - (textRect.width / 2.0),
                    (WINDOW_Y as f32 / 2.0) - (textRect.height / 2.0));
                println!("{:?}", text.get_position());
                text.set_color(&Color::red());
                window.draw(&text);

                window.draw(&rect);
            },
            Phase::LevelComplete => {
            },
            _ => {},
        }
        
        // Display things on screen
        window.display();
    }
}

fn draw_grid(window: &mut RenderWindow) {
    for grid_x in 0..GRID_SIZE {
        for grid_y in 0..GRID_SIZE {
            let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape!");
            rect.set_position2f((grid_x as f32) * (SQUARE_SIZE + GRIDLINE_WIDTH) + PADDINGF, (grid_y as f32) * (SQUARE_SIZE + GRIDLINE_WIDTH) + PADDINGF);
            rect.set_size(&Vector2f{x: SQUARE_SIZE, y: SQUARE_SIZE});
            rect.set_fill_color(&Color::black());
            rect.set_outline_color(&Color::white());
            rect.set_outline_thickness(GRIDLINE_WIDTH);

            window.draw(&rect);
        }
    }
}

fn draw_status_bar(window: &mut RenderWindow, game_state: &GameState, font: &Font) {
    let mut text = Text::new_init(
        &format!("Level: {}   Score: {}   Time: {}", game_state.level, game_state.score, game_state.clock.get_elapsed_time().as_seconds()), font, 32)
        .expect("Failed to render font.");
    text.set_color(&Color::white());
    text.set_position2f(PADDINGF, (WINDOW_Y - 32) as f32);
    window.draw(&text);
}
