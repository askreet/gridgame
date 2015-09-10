use std::f32;

use sfml::audio::{Sound, SoundBuffer};
use sfml::graphics::{RectangleShape, RenderTarget, RenderWindow};
use sfml::system::Clock;
use sfml::traits::drawable::Drawable;

use rand;
use rand::distributions::{IndependentSample, Range};

use na;
use na::{Iso2, Vec2};

use ncollide::bounding_volume;
use ncollide::broad_phase::BroadPhase;
use ncollide::broad_phase::DBVTBroadPhase;

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
    pub assets: &'a Assets,
    pub level: i8,
    pub score: i8,
    pub player: Piece,
    pub enemies: Vec<Piece>,
    pub treasures: Vec<Piece>,
    pub phase: Phase,
    pub clock: Clock,
    pub game_over_clock: Option<Clock>,

    pub soundboard: Soundboard,

    pub debug_ticks: bool,
    pub debug_loop: bool,

    pub game_over_curtain: RectangleShape<'a>,

    last_moved_enemies: i32,
    last_placed_treasure: i32,
    last_enemy_spawn: i32,
}

impl<'a> GameState<'a> {
    pub fn new(assets: &'a Assets) -> GameState<'a> {
        let mut curtain = RectangleShape::new().expect("Could not allocate RectangleShape!");
        curtain.set_size2f(WINDOW_X as f32, WINDOW_Y as f32);
        
        GameState {
            assets: assets,
            level: 1,
            score: 0,
            player: Piece::new(45., 55., assets.t_player.clone()),
            enemies: Vec::new(),
            treasures: Vec::new(),
            phase: Phase::Playing,
            clock: Clock::new(),
            game_over_clock: None,

            soundboard: Soundboard::new(assets),
            
            debug_ticks: false,
            debug_loop: false,

            game_over_curtain: curtain,

            last_moved_enemies: 0,
            last_placed_treasure: 0,
            last_enemy_spawn: 0,
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
                Entity::Nothing => {
                    enemy.vel = desired_move;
                },
                _ => {}
            }
            new_enemies.push(enemy);
        }

        self.enemies = new_enemies;
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
    
    pub fn game_over(&mut self) {
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

    pub fn update(&mut self) {
        let now = self.game_timer();
        
        if now - self.last_moved_enemies >= 1000 {
            self.move_enemies();
            self.last_moved_enemies = now;
        }

        if now - self.last_placed_treasure >= 2000 && self.treasures.len() < NUM_TREASURES {
            let point = self.random_free_location();
            self.treasures.push(Piece::new(point.x, point.y, self.assets.t_treasure.clone()));

            self.last_placed_treasure = now;
        }

        if now - self.last_enemy_spawn >= 500 && self.enemies.len() < NUM_ENEMIES {
            let point = self.random_free_location();
            self.enemies.push(Piece::new(point.x, point.y, self.assets.t_enemy.clone()));

            self.last_enemy_spawn = now;
        }

        // TODO: Traits or something?
        for enemy in &mut self.enemies {
            enemy.update();
        }

        self.player.update();

        for treasure in &mut self.treasures {
            treasure.update();
        }

        // Collision detection
        self.do_collision_detection();
    }

    fn do_collision_detection(&mut self) {
        // TODO: We shouldn't be building this up each time and instead just adding/removing
        //       elements with defered_add / defered_remove from a heap-allocated BroadPhase, probably.
        // TODO: What does 0.2 mean?
        let mut bf = DBVTBroadPhase::new(0.2, true);

        let player_bv = bounding_volume::aabb(&self.player.get_ncol_shape(),
                                              &Iso2::new(self.player.get_ncol_vec(), na::zero()));
        bf.defered_add(0, player_bv.clone(), (Entity::Player, 0));
        let mut ctr = 0;
        for (i, enemy) in self.enemies.iter().enumerate() {
            bf.defered_add(ctr+1, bounding_volume::aabb(&enemy.get_ncol_shape(),
                                                        &Iso2::new(enemy.get_ncol_vec(), na::zero())), (Entity::Enemy, i));

            ctr += 1;
        }

        for (i, treasure) in self.treasures.iter().enumerate() {
            bf.defered_add(ctr+1, bounding_volume::aabb(&treasure.get_ncol_shape(),
                                                        &Iso2::new(treasure.get_ncol_vec(), na::zero())), (Entity::Treasure, i));

            ctr += 1;
        }

        // TODO: Docs say this "avoids self collisions", but output from the below call to
        //       interferences with bounding volume always contain 1 result.
        bf.update(&mut |a, b| *a != *b, &mut |_, _, _| { });

        let mut collisions = Vec::new();

        bf.interferences_with_bounding_volume(&player_bv, &mut collisions);

        if collisions.len() > 1 {
            for collision in collisions {
                match *collision {
                    (Entity::Player, _) => {} // We don't care.
                    (Entity::Enemy, _) => {
                        self.game_over();
                    },
                    (Entity::Treasure, i) => {
                        self.score += 1;

                        // FIXME: This shifts items to the left, which could result in a panic situation if you gather
                        //        two treasures at once.
                        self.treasures.remove(i);
                    },
                    _ => {},
                }
            }
        }
    }
    
    fn random_free_location(&self) -> Vec2<f32> {
        loop {
            let pos = random_location();
            if dist(self.player.pos, pos) > 250.0 {
                return pos
            }
        }
    }

    pub fn reset(&mut self) {
        self.enemies.clear();
        self.treasures.clear();
        self.score = 0;
        self.level = 1;
        self.player.pos = Vec2::new(55.0, 45.0);
        self.phase = Phase::Playing;
        self.clock.restart();
        self.last_placed_treasure = 0;
        self.last_moved_enemies = 0;
        self.last_enemy_spawn = 0;
    }

    pub fn render<RT: RenderTarget>(&mut self, target: &mut RT, lag: f32) {
        for enemy in &self.enemies { enemy.render(target, lag); }
        for treasure in &self.treasures { treasure.render(target, lag); }
        self.player.render(target, lag);
    }
}

impl<'a> Drawable for GameState<'a> {
    fn draw<RT: RenderTarget>(&self, target: &mut RT) {
        for enemy in &self.enemies {
            target.draw(enemy);
        }
        for treasure in &self.treasures {
            target.draw(treasure);
        }
        target.draw(&self.player);
    }
}

fn random_location() -> Vec2<f32> {
    Vec2::new(random_upto(PLAYAREA_X), random_upto(PLAYAREA_Y))
}

fn random_upto(max: f32) -> f32 {
    let between = Range::new(0.0, max);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

#[inline]
fn dist(p1: Vec2<f32>, p2: Vec2<f32>) -> f32 {
    let xd = p1.x - p2.x;
    let yd = p1.y - p2.y;
    f32::sqrt(xd * xd + yd * yd)
}

