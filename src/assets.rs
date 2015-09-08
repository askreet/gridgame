use std::cell::RefCell;
use std::rc::Rc;

use sfml::audio::rc::Sound;
use sfml::audio::SoundBuffer;
use sfml::graphics::{Font, Texture};

pub struct Assets {
    pub t_player: Rc<Texture>,
    pub t_enemy: Rc<Texture>,
    pub t_treasure: Rc<Texture>,

    pub f_dosis_m: Rc<Font>,

    pub sb_pickup: Rc<RefCell<SoundBuffer>>,
}

pub fn load() -> Assets {
    let sb_pickup = load_sound_buffer("data/pickup.wav");
    
    Assets {
        t_player: load_texture("data/player-scaled.png"),
        t_enemy: load_texture("data/enemy.png"),
        t_treasure: load_texture("data/treasure.png"),

        f_dosis_m: load_font("data/Dosis/Dosis-Medium.ttf"),

        sb_pickup: sb_pickup.clone(),
    }
}

fn load_texture(filename: &str) -> Rc<Texture> {
    Rc::new(Texture::new_from_file(filename)
        .expect(&format!("Cannot load file: {}!", filename)))
}

fn load_sound_buffer(filename: &str) -> Rc<RefCell<SoundBuffer>> {
    Rc::new(RefCell::new(SoundBuffer::new(filename)
        .expect(&format!("Cannot load file: {}!", filename))))
}

fn load_font(filename: &str) -> Rc<Font> {
    Rc::new(Font::new_from_file(filename)
        .expect(&format!("Cannot load font: {}!", filename)))
}

pub struct Soundboard {
    pub s_pickup: Sound,
}

impl<'a> Soundboard {
    pub fn new(assets: &Assets) -> Soundboard {
        Soundboard {
            s_pickup: Sound::new_with_buffer(assets.sb_pickup.clone()).expect("Could not create Sound!"),
        }
    }
}
