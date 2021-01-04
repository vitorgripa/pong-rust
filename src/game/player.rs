const PLAYER_WIDTH: f64 = 5.0;
const PLAYER_HEIGHT: f64 = 60.0;

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
    pub fn new(position_x: f64, number: u8) -> Self {
        Self {
            position_y: 0.0 - PLAYER_HEIGHT / 2.0,
            position_x,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            color: [1.0, 1.0, 1.0, 1.0],
            score: 0,
            number
        }
    }

    pub fn make_point(&mut self) {
        self.score += 1;
    }

    pub fn reset_score(&mut self) {
        self.score = 0;
    }

    pub fn reset_position(&mut self) {
        match self.number {
            1 => {self.position_x = 250.0},
            2 => {self.position_x = -250.0},
            _ => {}
        }

        self.position_y = 0.0 - self.height / 2.0 
    }

    pub fn move_up(&mut self) {
        self.position_y -= 5.0;
    }

    pub fn move_down(&mut self) {
        self.position_y += 5.0;
    }
}