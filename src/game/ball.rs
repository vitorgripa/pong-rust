const BALL_HEIGHT: f64 = 10.0;
const BALL_WIDTH: f64 = 10.0;

pub struct Ball {
    pub position_y: f64,
    pub position_x: f64,
    pub width: f64,
    pub height: f64,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub color: [f32; 4]
}

impl Ball {

    pub fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            width: BALL_WIDTH,
            height: BALL_HEIGHT,
            acceleration_x: 1.0,
            acceleration_y: 1.0,
            color: [1.0, 1.0, 1.0, 1.0,]
        }
    }

    pub fn check_collision(&mut self, obj_x1: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) {
        let ball_x1: &f64 = &self.position_x;
        let ball_x2: &f64 = &(self.position_x + self.width);
        let ball_y1: &f64 = &self.position_y;
        let ball_y2: &f64 = &(self.position_y + self.height);

        if ball_x1 == obj_x2 && ball_y1 >= obj_y1 && ball_y2 <= obj_y2 || ball_x2 == obj_x1 && ball_y1 >= obj_y1 && ball_y2 <= obj_y2 {
            self.acceleration_x *= -1.0;
        } else if ball_y1 == obj_y2 && ball_x1 >= obj_x1 && ball_x2 <= obj_x2 || ball_y2 == obj_y1 && ball_x1 >= obj_x1 && ball_x2 <= obj_x2 {
            self.acceleration_y *= -1.0;
        }
    }

    pub fn reset_position(&mut self) {
        self.position_x = 0.0 - self.width / 2.0;
        self.position_y = 0.0 - self.height / 2.0;
    }
}
