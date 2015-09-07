extern crate sfml;

use std::fmt;

use sfml::graphics::Sprite;
use sfml::graphics::Texture;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;

use ncollide::shape::Cuboid;

use na::Vec2;

use constants::*;

pub struct Piece<'a> {
    pub x: f32,
    pub y: f32,
    pub texture: &'a Texture,
}

impl<'a> Piece<'a> {
    pub fn new(x: f32, y: f32, texture: &Texture) -> Piece {
        Piece {
            x: x,
            y: y,
            texture: texture,
        }
    }
    
    // Returns false if the move is out of bounds.
    pub fn move_(&mut self, x: f32, y: f32) -> bool {
        if self.x + x < 0.0 ||
            self.x + x > PLAYAREA_X ||
            self.y + y < 0.0 ||
            self.y + y > PLAYAREA_Y {
                false
            } else {
                self.x += x;
                self.y += y;
                    
                true
            }
    }

    // Scale and draw the piece on the game board.
    pub fn draw(&self, target: &mut RenderWindow) {
        let mut sprite = Sprite::new_with_texture(self.texture)
            .expect("Could not create Sprite!");

        sprite.scale2f(
            (PIECE_SIZE / self.texture.get_size().x as f32),
            (PIECE_SIZE / self.texture.get_size().y as f32));
        sprite.set_position2f(self.x + PADDING, self.y + PADDING);
        target.draw(&sprite);
    }

    // pub fn get_size() -> Vec2<f32> {
        
    // }
    
    // TODO: Store this for longer if possible.
    pub fn get_ncol_shape(&self) -> Cuboid<Vec2<f32>> {
        Cuboid::new(Vec2::new(self.texture.get_size().x as f32 / 2.0,
                              self.texture.get_size().y as f32 / 2.0))
    }

    pub fn get_ncol_vec(&self) -> Vec2<f32> {
        // TODO: I'm assuming ncollide expects a cen
        Vec2::new(self.texture.get_size().x as f32 / 2.0,
                  self.texture.get_size().y as f32 / 2.0)
    }
}

impl<'a> fmt::Debug for Piece<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
