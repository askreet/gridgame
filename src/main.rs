//! Example from SFML: Shape

extern crate sfml;

use sfml::graphics::{RenderWindow, Color, Shape, RenderTarget, Vertex, VertexArray, PrimitiveType, RectangleShape, Texture, Sprite};
use sfml::window::{VideoMode, ContextSettings, event, Close};
use sfml::window::keyboard::Key;
use sfml::system::Vector2f;

use std::thread;

mod constants;
use constants::*;
mod piece;
use piece::{Piece};

fn main() {
    // Create the window of the application
    let setting = ContextSettings::default();
    let mut window = RenderWindow::new(VideoMode::new_init(WINDOW_X, WINDOW_Y, 32), "GridGame", Close, &setting)
        .expect("Cannot create a new RenderWindow");
    window.set_vertical_sync_enabled(true);

    let player_texture = Texture::new_from_file("data/player-scaled.png")
        .expect("Cannot load player-scaled.png!");
    let mut player = Piece::new(5, GRID_SIZE as i8 - 1, &player_texture);

    let enemy_texture = Texture::new_from_file("data/enemy.png")
        .expect("Cannot load enemy.png!");
    
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                event::KeyPressed{code, ..} => match code {
                    Key::Escape => {
                        window.close();
                        break;
                    },
                    Key::Up => { player.move_(0, -1); },
                    Key::Down => { player.move_(0, 1); }, 
                    Key::Left => { player.move_(-1, 0); },
                    Key::Right => { player.move_(1, 0); },
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
        player.draw(&mut window);
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

