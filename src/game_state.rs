use sfml::graphics::{RenderWindow, Texture};
use sfml::system::Clock;

use rand;
use rand::distributions::{IndependentSample, Range};

use piece::Piece;

use constants::*;

#[derive(PartialEq)]
pub enum Phase {
    Playing,
    PlayerLost,
    LevelComplete,
}

pub enum Entity {
    Player,
    Enemy,
    Treasure,
    Nothing,
}

pub struct GameState<'a> {
    pub level: i8,
    pub score: i8,
    pub player: Piece<'a>,
    pub enemies: Vec<Piece<'a>>,
    pub phase: Phase,
    pub clock: Clock,
    pub last_tick: i32,
    pub game_over_clock: Option<Clock>,
}

impl<'a> GameState<'a> {
    pub fn new(player_texture: &'a Texture, enemy_texture: &'a Texture, treasure_texture: &'a Texture) -> GameState<'a> {
        GameState {
            level: 1,
            score: 0,
            player: Piece::new(9, 5, player_texture),
            enemies: vec![
                Piece::new(1, 1, &enemy_texture),
                Piece::new(2, 2, &enemy_texture),
                Piece::new(9, 1, &enemy_texture),
                Piece::new(8, 2, &enemy_texture),
                ],
            phase: Phase::Playing,
            clock: Clock::new(),
            last_tick: 0,
            game_over_clock: None,
        }
    }
    
    pub fn move_player(&mut self, x: i8, y: i8) {
        if self.phase != Phase::Playing {
            return;
        }
        // Check for enemy collision
        let mut game_over = false;
        for enemy in &self.enemies {
            if (self.player.x + x) == enemy.x && (self.player.y + y) == enemy.y {
                game_over = true;
            }
        }
        if game_over {
            self.game_over();
            return;
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
        while self.game_timer() > self.last_tick + TICK_FREQ_MS {
            self.last_tick += TICK_FREQ_MS;
        }

        if self.last_tick % 4 == 0 {
            self.move_enemies();
        }
    }
}
