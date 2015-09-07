pub const SQUARE_SIZE: f32 = 96.;
pub const GRID_SIZE: u32 = 10;
pub const GRIDLINE_WIDTH: f32 = 2.;
pub const PADDING: u32 = 4;
pub const PADDINGF: f32 = 4.;
pub const WINDOW_X: u32 = ((SQUARE_SIZE as u32 + GRIDLINE_WIDTH as u32) * GRID_SIZE) + PADDING * 2;
pub const WINDOW_Y: u32 = WINDOW_X + 32 + PADDING; // To show clock / score.
pub const ENEMY_MOVE_FREQ: f32 = 0.4;

pub const TICK_FREQ_MS: i32 = 250;
pub const NUM_TREASURES: usize = 2;
pub const NUM_ENEMIES: usize = 4;
