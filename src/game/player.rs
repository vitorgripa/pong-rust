use std::error::Error;

const PLAYER_WIDTH: f64 = 5.0;
const PLAYER_HEIGHT: f64 = 60.0;
const PLAYER_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Player {
    pub number: u8,
    pub position_y: f64,
    pub position_x: f64,
    pub width: f64,
    pub height: f64,
    pub color: [f32; 4],
    pub score: u32,
}

impl Player {
    pub fn new(position_x: f64, number: u8) -> Result<Player, Box<dyn Error>> {
        let position_y: f64 = 0.0 - PLAYER_HEIGHT / 2.0;
        let width: f64 = PLAYER_WIDTH;
        let height: f64 = PLAYER_HEIGHT;
        let color: [f32; 4] = PLAYER_COLOR;
        let score: u32 = 0;

        Ok(
            Player {
                position_y,
                position_x,
                width,
                height,
                color,
                score,
                number
            }
        )
    }

    pub fn make_point(&mut self) {
        self.score += 1;
    }

    pub fn move_up(&mut self) {
        self.position_y -= 5.0;
    }

    pub fn move_down(&mut self) {
        self.position_y += 5.0;
    }
}