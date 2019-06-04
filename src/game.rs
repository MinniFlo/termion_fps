pub mod structs;
pub mod renderLogic;

use structs::{Player, Window};

pub struct GameState {
    pub player: Player,
    pub map_win: Window,
    pub render_win: Window,
    pub map_vec: Vec<Vec<char>>,
    pub render_vec: Vec<Vec<char>>,
    pub floor_texture: Vec<char>,
    pub render_dist: u32
}

