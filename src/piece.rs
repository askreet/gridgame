extern crate sfml;

use std::fmt;
use std::rc::Rc;

use sfml::graphics::Color;
use sfml::graphics::Sprite;
use sfml::graphics::Texture;
use sfml::graphics::RectangleShape;
use sfml::graphics::RenderTarget;
use sfml::graphics::RenderWindow;

use ncollide::shape::Cuboid;

use na::Vec2;

use constants::*;

pub struct Piece {
    pub pos: Vec2<f32>,
    pub size: Vec2<f32>,
    pub texture: Rc<Texture>,
}

impl Piece {
    pub fn new(x: f32, y: f32, texture: Rc<Texture>) -> Piece {
        Piece {
            pos: Vec2::new(x, y),
            size: Vec2::new(PIECE_SIZE, PIECE_SIZE),
            texture: texture,
        }
    }
    
    // Returns false if the move is out of bounds.
    pub fn move_by(&mut self, pos: Vec2<f32>) -> bool {
        let target = self.pos + pos;
        let bottom_right = self.pos + self.size;
        if target.x < 0.0 || target.y < 0.0 ||
            bottom_right.x > PLAYAREA_X || bottom_right.y > PLAYAREA_Y {
                false
            } else {
                self.pos = self.pos + pos;
                    
                true
            }
    }

    // Scale and draw the piece on the game board.
    pub fn draw(&self, target: &mut RenderWindow) {
        let mut sprite = Sprite::new_with_texture(&*self.texture)
            .expect("Could not create Sprite!");

        sprite.scale2f(
            (PIECE_SIZE / self.texture.get_size().x as f32),
            (PIECE_SIZE / self.texture.get_size().y as f32));

        let pos = self.pos + Vec2::new(PADDING, PADDING);
        sprite.set_position2f(pos.x, pos.y);
        target.draw(&sprite);
    }

    pub fn draw_collision_shape(&self, target: &mut RenderWindow) {
        let mut rect = RectangleShape::new().expect("Could not allocate RectangleShape.");
        rect.set_size2f(self.size.x, self.size.y);
        rect.set_position2f(self.pos.x, self.pos.y);
        rect.set_outline_color(&Color::white());
        rect.set_outline_thickness(2.0);
        rect.set_fill_color(&Color::transparent());

        target.draw(&rect);
    }

    pub fn get_ncol_shape(&self) -> Cuboid<Vec2<f32>> {
        Cuboid::new(Vec2::new(self.size.x / 2.0, self.size.y / 2.0))
    }

    pub fn get_ncol_vec(&self) -> Vec2<f32> {
        // TODO: I'm assuming ncollide expects a center point.
        Vec2::new(self.size.x / 2.0, self.size.y / 2.0)
    }
}
