
use super::*;

pub struct Selector {
    pub current: u32,
    pub total_games: u32,
    pub y_value: f32,
}

impl Default for Selector {
    fn default() -> Self {
        Self { current: 0, total_games: 0, y_value: 0.0 }
    }
}

