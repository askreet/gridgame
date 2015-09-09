extern crate sfml;
extern crate rand;
extern crate ncollide;
extern crate nalgebra as na;

use std::sync::Mutex;

use sfml::graphics::{RenderWindow, Color, RenderTarget, RectangleShape, Font, Text};
use sfml::window::{VideoMode, ContextSettings, event, Close};
use sfml::window::keyboard::Key;
use sfml::traits::drawable::Drawable;

use na::{Iso2, Vec2};

use ncollide::broad_phase::BroadPhase;
use ncollide::broad_phase::DBVTBroadPhase;
use ncollide::bounding_volume;

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

    let mut previous: i32 = game_state.game_timer();
    let mut lag: i32 = 0;

    while window.is_open() {
        let current: i32 = game_state.game_timer();
        let elapsed: i32 = current - previous;

        previous = current;
        lag += elapsed;

        handle_input(&mut window, &mut game_state);

        while lag >= MS_PER_UPDATE {
            update(&mut game_state);
            lag -= MS_PER_UPDATE;
        }

        render(&mut window, &mut game_state);
    }
}

fn handle_input(window: &mut RenderWindow, state: &mut GameState) {
    for event in window.events() {
        match event {
            event::Closed => window.close(),
            event::KeyPressed{code, ..} => match code {
                Key::Escape => {
                    window.close();
                    break;
                },
                Key::Up => state.move_player(Vec2::new(0., -10.)),
                Key::Down => state.move_player(Vec2::new(0., 10.)),
                Key::Left => state.move_player(Vec2::new(-10., 0.)),
                Key::Right => state.move_player(Vec2::new(10., 0.)),
                Key::Space | Key::Return => {
                    if state.phase == Phase::PlayerLost {
                        state.reset();
                    }
                },
                Key::F1 => {
                    state.debug_ticks = !state.debug_ticks;
                },
                Key::F2 => {
                    state.debug_loop = !state.debug_loop;
                }
                _ => {}
            },
            _ => {}
        };
    }
}

// Perform updates to the game state based on movement forward in time.
fn update(state: &mut GameState) {
    match state.phase {
        Phase::Playing => {
            if state.check_tick() { state.tick() }

            let start_col = state.game_timer();

            // TODO: What does 0.2 mean?
            let mut bf = DBVTBroadPhase::new(0.2, true);

            bf.defered_add(0, bounding_volume::aabb(&state.player.get_ncol_shape(),
                                                    &Iso2::new(state.player.get_ncol_vec(), na::zero())), 0);
            let mut ctr = 0;
            for enemy in &state.enemies {
                bf.defered_add(ctr+1, bounding_volume::aabb(&enemy.get_ncol_shape(),
                                                            &Iso2::new(enemy.get_ncol_vec(), na::zero())), ctr+1);

                ctr += 1;
            }

            bf.update(&mut |a, b| *a != *b, &mut |_, _, _| { });

            if bf.num_interferences() > 0 {
                println!("Collision took {}ms, found {} interferences.", state.game_timer() - start_col, bf.num_interferences());
            }
        }
        Phase::PlayerLost => {
        },
        // Phase::LevelComplete => {},
    }
}

fn render(window: &mut RenderWindow, state: &mut GameState) {
    // Clear the window
    window.clear(&Color::black());

    // Draw the playarea
    draw_playarea(window);

    match state.phase {
        Phase::Playing => {
            draw_status_bar(window, &state);
            window.draw(state);
        },
        Phase::PlayerLost => {
            // Display gradient / game over based on time since loss.
            window.draw(state);

            let time = state.ms_since_dead();
            let alpha = linear_tween(time, 0, state.game_over_curtain.get_fill_color().alpha, 1000);

            state.game_over_curtain.set_fill_color(&Color::new_rgba(0, 0, 0, alpha));

            let mut text = Text::new_init("GAME OVER", &state.assets.f_dosis_m, 96)
                .expect("Failed to render text!");
            let text_rect = text.get_local_bounds();
            text.set_position2f(
                (WINDOW_X as f32 / 2.0) - (text_rect.width / 2.0),
                (WINDOW_Y as f32 / 2.0) - (text_rect.height / 2.0));
            text.set_color(&Color::red());
            window.draw(&text);

            window.draw(&state.game_over_curtain);
        }
    }

    // Display things on screen
    window.display();
}

fn draw_playarea(window: &mut RenderWindow) {
    let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape!");
    rect.set_position2f(PADDING as f32, PADDING as f32);
    rect.set_size2f(PLAYAREA_X as f32, PLAYAREA_Y as f32);
    rect.set_fill_color(&Color::new_rgb(64, 64, 64));

    window.draw(&rect);
}

fn draw_status_bar(window: &mut RenderWindow, game_state: &GameState) {
    let mut text = Text::new_init(
        &format!("Level: {}   Score: {}   Time: {:.2}", game_state.level, game_state.score, game_state.clock.get_elapsed_time().as_seconds()), &game_state.assets.f_dosis_m, 32)
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
