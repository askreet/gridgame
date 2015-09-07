use sfml::audio::{SoundBuffer};
use sfml::graphics::{Font, Texture};

pub struct Assets {
    pub t_player: Texture,
    pub t_enemy: Texture,
    pub t_treasure: Texture,

    pub f_dosis_m: Font,

    pub sb_pickup: SoundBuffer,
}

pub fn load() -> Assets {
    Assets {
        t_player: load_texture("data/player-scaled.png"),
        t_enemy: load_texture("data/enemy.png"),
        t_treasure: load_texture("data/treasure.png"),

        f_dosis_m: load_font("data/Dosis/Dosis-Medium.ttf"),
        
        sb_pickup: load_sound("data/pickup.wav"),
    }
}

fn load_texture(filename: &str) -> Texture {
    Texture::new_from_file(filename)
        .expect(&format!("Cannot load file: {}!", filename))
}

fn load_sound(filename: &str) -> SoundBuffer {
    SoundBuffer::new(filename)
        .expect(&format!("Cannot load file: {}!", filename))
}

fn load_font(filename: &str) -> Font {
    Font::new_from_file(filename)
        .expect(&format!("Cannot load font: {}!", filename))
}
