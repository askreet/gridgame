//! Example from SFML: Shape

extern crate sfml;
extern crate rand;

use sfml::graphics::{RenderWindow, Color, RenderTarget, RectangleShape, Font, Text};
use sfml::window::{VideoMode, ContextSettings, event, Close};
use sfml::window::keyboard::Key;
use sfml::system::Vector2f;

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
                    Key::Up => game_state.move_player(0, -1),
                    Key::Down => game_state.move_player(0, 1),
                    Key::Left => game_state.move_player(-1, 0),
                    Key::Right => game_state.move_player(1, 0),
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
        draw_grid(&mut window);

        match game_state.phase {
            Phase::Playing => {
                if game_state.check_tick() {
                    game_state.tick();
                }
                
                draw_status_bar(&mut window, &game_state, &assets.f_dosis_m);
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
                let mut text = Text::new_init("GAME OVER", &assets.f_dosis_m, SQUARE_SIZE as u32 * 2)
                    .expect("Failed to render text!");
                let text_rect = text.get_local_bounds();
                text.set_position2f(
                    (WINDOW_X as f32 / 2.0) - (text_rect.width / 2.0),
                    (WINDOW_Y as f32 / 2.0) - (text_rect.height / 2.0));
                text.set_color(&Color::red());
                window.draw(&text);

                window.draw(&rect);
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

fn draw_grid(window: &mut RenderWindow) {
    for grid_x in 0..GRID_SIZE {
        for grid_y in 0..GRID_SIZE {
            let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape!");
            rect.set_position2f((grid_x as f32) * (SQUARE_SIZE + GRIDLINE_WIDTH) + PADDINGF, (grid_y as f32) * (SQUARE_SIZE + GRIDLINE_WIDTH) + PADDINGF);
            rect.set_size(&Vector2f{x: SQUARE_SIZE, y: SQUARE_SIZE});
            rect.set_fill_color(&Color::black());
            rect.set_outline_color(&Color::new_rgb(64, 64, 64));
            rect.set_outline_thickness(GRIDLINE_WIDTH);

            window.draw(&rect);
        }
    }
}

fn draw_status_bar(window: &mut RenderWindow, game_state: &GameState, font: &Font) {
    let mut text = Text::new_init(
        &format!("Level: {}   Score: {}   Time: {:.2}", game_state.level, game_state.score, game_state.clock.get_elapsed_time().as_seconds()), font, 32)
        .expect("Failed to render font.");
    text.set_color(&Color::white());
    text.set_position2f(PADDINGF, (WINDOW_Y - 32) as f32);
    window.draw(&text);
}
