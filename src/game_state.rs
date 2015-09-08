use sfml::audio::{Sound, SoundBuffer};
use sfml::graphics::{RenderWindow};
use sfml::system::Clock;

use rand;
use rand::distributions::{IndependentSample, Range};

use na::Vec2;

use piece::Piece;
use assets::{Assets, Soundboard};

use constants::*;

#[derive(PartialEq)]
pub enum Phase {
    Playing,
    PlayerLost,
    // LevelComplete,
}

#[derive(PartialEq)]
pub enum Entity {
    Player,
    Enemy,
    Treasure,
    Nothing,
}

pub struct GameState<'a> {
    assets: &'a Assets,
    pub level: i8,
    pub score: i8,
    pub player: Piece,
    pub enemies: Vec<Piece>,
    pub treasures: Vec<Piece>,
    pub phase: Phase,
    pub clock: Clock,
    pub last_tick: i32,
    pub game_over_clock: Option<Clock>,

    pub soundboard: Soundboard,

    pub debug_ticks: bool,
    pub debug_loop: bool,
}

impl<'a> GameState<'a> {
    pub fn new(assets: &'a Assets) -> GameState<'a> {
        GameState {
            assets: assets,
            level: 1,
            score: 0,
            player: Piece::new(45., 55., assets.t_player.clone()),
            enemies: Vec::new(),
            treasures: Vec::new(),
            phase: Phase::Playing,
            clock: Clock::new(),
            last_tick: 0,
            game_over_clock: None,

            soundboard: Soundboard::new(assets),
            
            debug_ticks: false,
            debug_loop: false,
        }
    }
    
    pub fn move_player(&mut self, chg: Vec2<f32>) {
        if self.phase != Phase::Playing {
            return;
        }

        let target = self.player.pos + chg;
        match self.entity_at_square(target) {
            Entity::Enemy => { self.game_over(); return; },
            
            Entity::Treasure => {
                self.soundboard.s_pickup.play();
                self.score += 1;
                // Keep all other treasures.
                self.treasures.retain(|t| { t.pos != target });
            },
            _ => {},
        }

        // Check for win condition
        // NOT IMPLEMENTED

        // Move player
        self.player.move_by(chg);
    }

    pub fn move_enemies(&mut self) {
        let mut new_enemies: Vec<Piece> = Vec::new();
        
        while let Some(mut enemy) = self.enemies.pop() {
            let desired_move = self.random_movement();
            match self.entity_at_square(enemy.pos + desired_move) {
                Entity::Player => self.game_over(),
                Entity::Nothing => { enemy.move_by(desired_move); },
                _ => {}
            }
            new_enemies.push(enemy);
        }

        self.enemies = new_enemies;
    }

    pub fn draw_all(&self, window: &mut RenderWindow) {
        for enemy in &self.enemies {
            enemy.draw(window);
            enemy.draw_collision_shape(window);
        }
        for treasure in &self.treasures {
            treasure.draw(window);
            treasure.draw_collision_shape(window);
        }
        self.player.draw(window);
        self.player.draw_collision_shape(window);
    }

    pub fn entity_at_square(&self, pos: Vec2<f32>) -> Entity {
        if self.player.pos == pos {
            return Entity::Player;
        }

        for enemy in &self.enemies {
            if enemy.pos == pos {
                return Entity::Enemy;
            }
        }

        for treasure in &self.treasures {
            if treasure.pos == pos{
                return Entity::Treasure;
            }
        }

        return Entity::Nothing;
    }
    
    fn game_over(&mut self) {
        self.phase = Phase::PlayerLost;
        self.game_over_clock = Some(Clock::new());
    }

    fn random_movement(&self) -> Vec2<f32> {
        let between = Range::new(0, 4);
        let mut rng = rand::thread_rng();
        match between.ind_sample(&mut rng) {
            0 => Vec2::new(0., -1.),
            1 => Vec2::new(0., 1.),
            2 => Vec2::new(-1., 0.),
            3 => Vec2::new(1., 0.),
            _ => Vec2::new(0., 0.), // This shouldn't happen.
        }
    }

    pub fn ms_since_dead(&self) -> i32 {
        match self.game_over_clock {
            Some(ref clock) => clock.get_elapsed_time().as_milliseconds(),
            None => 0,
        }
    }

    pub fn game_timer(&self) -> i32 {
        self.clock.get_elapsed_time().as_milliseconds()
    }
    
    pub fn check_tick(&self) -> bool {
        self.game_timer() > self.last_tick + TICK_FREQ_MS
    }
    
    pub fn tick(&mut self) {
        let start_at = self.game_timer();
        
        while self.game_timer() > self.last_tick + TICK_FREQ_MS {
            self.last_tick += TICK_FREQ_MS;
        }

        if self.last_tick % 4 == 0 {
            self.move_enemies();
        }

        if self.last_tick > 2000 && self.treasures.len() < NUM_TREASURES {
            let point = self.random_free_location();
            self.treasures.push(Piece::new(point.0, point.1, self.assets.t_treasure.clone()));
        }

        if self.last_tick > 1000 && self.enemies.len() < NUM_ENEMIES {
            let point = self.random_free_location();
            self.enemies.push(Piece::new(point.0, point.1, self.assets.t_enemy.clone()));
        }

        let end_at = self.game_timer();
        if self.debug_ticks {
            println!("Tick at {}ms took {}ms total, delayed by {}ms.",
                     self.last_tick,
                     end_at - start_at,
                     start_at - self.last_tick);
        }
    }

    fn random_free_location(&self) -> (f32, f32) {
        // LOL.
        random_location()
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
        self.treasures.clear();
        self.score = 0;
        self.level = 1;
        self.player.pos = Vec2::new(55.0, 45.0);
        self.phase = Phase::Playing;
        self.last_tick = 0;
        self.clock.restart();
    }
}

fn random_location() -> (f32, f32) {
    (random_upto(PLAYAREA_X), random_upto(PLAYAREA_Y))
}

fn random_upto(max: f32) -> f32 {
    let between = Range::new(0.0, max);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}
