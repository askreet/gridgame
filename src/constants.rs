// TODO: Implement scaling based on window resize. Also Retinas are a wtf.
// pub const GAME_SCALE: f32 = 1.2;
pub const PLAYAREA_X: f32 = 900.0;
pub const PLAYAREA_Y: f32 = 600.0;

pub const PADDING: f32 = 16.0;
pub const WINDOW_X: u32 = (PLAYAREA_X as u32) + (PADDING as u32 * 2) + 32; // Room for clock / score.
pub const WINDOW_Y: u32 = (PLAYAREA_Y as u32) + (PADDING as u32 * 2);

pub const PIECE_SIZE: f32 = 96.0;

pub const TICK_FREQ_MS: i32 = 250;

// TODO: These should increase with difficulty.
pub const NUM_TREASURES: usize = 2;
pub const NUM_ENEMIES: usize = 4;

pub const DEBUG_COLLISION: bool = true;

pub const MS_PER_UPDATE: i32 = 16;

pub const PLAYER_MOVE_SPEED: f32 = 1.50;
