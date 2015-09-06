extern crate sfml;

use std::fmt;

use sfml::graphics::Sprite;
use sfml::graphics::Texture;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;

use constants::*;

pub struct Piece<'a> {
    pub x: i8,
    pub y: i8,
    pub texture: &'a Texture,
}

impl<'a> Piece<'a> {
    pub fn new(x: i8, y: i8, texture: &Texture) -> Piece {
        Piece {
            x: x,
            y: y,
            texture: texture,
        }
    }
    
    // Returns false if the move is out of bounds.
    pub fn move_(&mut self, x: i8, y: i8) -> bool {
        if self.x + x < 0 ||
            self.x + x > (GRID_SIZE as i8 - 1) ||
            self.y + y < 0 ||
            self.y + y > (GRID_SIZE as i8 - 1) {
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
            ((SQUARE_SIZE as u32) / self.texture.get_size().x) as f32,
            ((SQUARE_SIZE as u32) / self.texture.get_size().y) as f32);
        sprite.set_position2f(
            (self.x as f32 * (SQUARE_SIZE + GRIDLINE_WIDTH)) + PADDINGF,
            (self.y as f32 * (SQUARE_SIZE + GRIDLINE_WIDTH)) + PADDINGF
                );
        target.draw(&sprite);
    }

}

impl<'a> fmt::Debug for Piece<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
