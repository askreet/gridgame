use sfml::graphics::{RenderWindow};
use sfml::system::Clock;

use rand;
use rand::distributions::{IndependentSample, Range};

use piece::Piece;
use assets::Assets;

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
    pub player: Piece<'a>,
    pub enemies: Vec<Piece<'a>>,
    pub treasures: Vec<Piece<'a>>,
    pub phase: Phase,
    pub clock: Clock,
    pub last_tick: i32,
    pub game_over_clock: Option<Clock>,

    pub debug_ticks: bool,
    pub debug_loop: bool,
}

impl<'a> GameState<'a> {
    pub fn new(assets: &'a Assets) -> GameState<'a> {
        GameState {
            assets: assets,
            level: 1,
            score: 0,
            player: Piece::new(4, 5, &assets.t_player),
            enemies: Vec::new(),
            treasures: Vec::new(),
            phase: Phase::Playing,
            clock: Clock::new(),
            last_tick: 0,
            game_over_clock: None,

            debug_ticks: false,
            debug_loop: false,
        }
    }
    
    pub fn move_player(&mut self, x: i8, y: i8) {
        if self.phase != Phase::Playing {
            return;
        }

        match self.entity_at_square(self.player.x + x, self.player.y + y) {
            Entity::Enemy => { self.game_over(); return; },
            
            Entity::Treasure => {
                self.score += 1;
                let (x, y) = (self.player.x + x, self.player.y + y);
                // Keep all other treasures.
                self.treasures.retain(|t| { t.x != x || t.y != y });
            },
            _ => {},
        }

        // Check for win condition
        // NOT IMPLEMENTED

        // Move player
        self.player.move_(x, y);
    }

    pub fn move_enemies(&mut self) {
        let mut new_enemies: Vec<Piece> = Vec::new();
        
        while let Some(mut enemy) = self.enemies.pop() {
            let desired_move = self.random_movement();
            match self.entity_at_square(enemy.x + desired_move.0, enemy.y + desired_move.1) {
                Entity::Player => self.game_over(),
                Entity::Nothing => { enemy.move_(desired_move.0, desired_move.1); },
                _ => {}
            }
            new_enemies.push(enemy);
        }

        self.enemies = new_enemies;
    }

    pub fn draw_all(&self, window: &mut RenderWindow) {
        for enemy in &self.enemies {
            enemy.draw(window);
        }
        for treasure in &self.treasures {
            treasure.draw(window);
        }
        self.player.draw(window);
    }

    pub fn entity_at_square(&self, x: i8, y: i8) -> Entity {
        if self.player.x == x && self.player.y == y {
            return Entity::Player;
        }

        for enemy in &self.enemies {
            if enemy.x == x && enemy.y == y {
                return Entity::Enemy;
            }
        }

        for treasure in &self.treasures {
            if treasure.x == x && treasure.y == y {
                return Entity::Treasure;
            }
        }

        return Entity::Nothing;
    }
    
    fn game_over(&mut self) {
        self.phase = Phase::PlayerLost;
        self.game_over_clock = Some(Clock::new());
    }

    fn random_movement(&self) -> (i8, i8) {
        let between = Range::new(0, 4);
        let mut rng = rand::thread_rng();
        match between.ind_sample(&mut rng) {
            0 => (0, -1),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (1, 0),
            _ => (0, 0), // This shouldn't happen.
        }
    }

    pub fn seconds_since_dead(&self) -> f32 {
        match self.game_over_clock {
            Some(ref clock) => clock.get_elapsed_time().as_seconds(),
            None => 0.0,
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
            let point = self.random_free_sq();
            self.treasures.push(Piece::new(point.0, point.1, &self.assets.t_treasure));
        }

        if self.last_tick > 1000 && self.enemies.len() < NUM_ENEMIES {
            let point = self.random_free_sq();
            self.enemies.push(Piece::new(point.0, point.1, &self.assets.t_enemy));
        }

        let end_at = self.game_timer();
        if self.debug_ticks {
            println!("Tick at {}ms took {}ms total, delayed by {}ms.",
                     self.last_tick,
                     end_at - start_at,
                     start_at - self.last_tick);
        }
    }

    fn random_free_sq(&self) -> (i8, i8) {
        let mut point: (i8, i8);
        loop {
            point = random_sq();
            if Entity::Nothing == self.entity_at_square(point.0, point.1) {
                break;
            }
        }
        point
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
        self.treasures.clear();
        self.score = 0;
        self.level = 1;
        self.player.x = 5;
        self.player.y = 4;
        self.phase = Phase::Playing;
        self.last_tick = 0;
        self.clock.restart();
    }
}

fn random_sq() -> (i8, i8) {
    let between = Range::new(0, GRID_SIZE);
    let mut rng = rand::thread_rng();
    (between.ind_sample(&mut rng) as i8, between.ind_sample(&mut rng) as i8)
}
