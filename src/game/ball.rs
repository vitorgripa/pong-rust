use std::error::Error;
use rand::*;

const BALL_HEIGHT: f64 = 10.0;
const BALL_WIDTH: f64 = 10.0;
const BALL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MAX_ACCELERATION: f64 = 5.0;
const ACCELERATION_INCREMENT: f64 = 1.15;

pub struct Ball {
    pub position_y: f64,
    pub position_x: f64,
    pub width: f64,
    pub height: f64,
    pub acceleration_x: f64,
    pub acceleration_y: f64,
    pub color: [f32; 4],
    pub angle: f64
}

impl Ball {

    pub fn new() -> Result<Ball, Box<dyn Error>> {
        let position_x: f64 = 0.0 - BALL_WIDTH / 2.0;
        let position_y: f64 = 0.0 - BALL_HEIGHT / 2.0;
        let width: f64 = BALL_WIDTH;
        let height: f64 = BALL_HEIGHT;
        let acceleration_x: f64 = 1.0;
        let acceleration_y: f64 = 1.0;
        let color: [f32; 4] = BALL_COLOR;
        let angle: f64 = Ball::randomize_angle();

        Ok(
            Ball {
                position_x,
                position_y,
                width,
                height,
                acceleration_x,
                acceleration_y,
                color,
                angle
            }
        )
    }

    pub fn reverse_x(&mut self) {
        self.acceleration_x *= -1.0;
    }

    pub fn reverse_y(&mut self) {
        self.acceleration_y *= -1.0;
    }

    pub fn increase_acceleration_x(&mut self) {
        if self.acceleration_x <= MAX_ACCELERATION && self.acceleration_x >= -MAX_ACCELERATION{
            self.acceleration_x *= ACCELERATION_INCREMENT;
        }
    }

    pub fn increase_acceleration_y(&mut self) {
        if self.acceleration_y <= MAX_ACCELERATION && self.acceleration_y >= -MAX_ACCELERATION {
            self.acceleration_y *= ACCELERATION_INCREMENT;
        }
    }

    pub fn check_touch_right(&self, ball_x1: &f64, ball_x2: &f64, ball_y1: &f64, ball_y2: &f64, obj_x1: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        if ball_x2 >= obj_x1 && ball_x1 < obj_x1 && ball_y1 <= obj_y2 && ball_y2 >= obj_y1{
            return true;
        }

        false
    }

    pub fn check_touch_left(&self, ball_x1: &f64, ball_x2: &f64, ball_y1: &f64, ball_y2: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        if ball_x1 <= obj_x2 && ball_x2 > obj_x2 && ball_y1 <= obj_y2 && ball_y2 >= obj_y1{
            return true;
        }

        false
    }

    pub fn check_touch_up(&self, ball_x1: &f64, ball_x2: &f64, ball_y1: &f64, ball_y2: &f64, obj_x1: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        if ball_y1 <= obj_y2 && ball_y2 > obj_y1 && ball_x1 <= obj_x2 && ball_x2 >= obj_x1{
            return true;
        }

        false
    }

    pub fn check_touch_down(&self, ball_x1: &f64, ball_x2: &f64, ball_y1: &f64, ball_y2: &f64, obj_x1: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        if ball_y2 >= obj_y1 && ball_y1 < obj_y2 && ball_x1 <= obj_x2 && ball_x2 >= obj_x1{
            return true;
        }

        false
    }

    pub fn check_collision(&mut self, obj_x1: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        let ball_x1: &f64 = &self.position_x;
        let ball_x2: &f64 = &(self.position_x + self.width);
        let ball_y1: &f64 = &self.position_y;
        let ball_y2: &f64 = &(self.position_y + self.height);

        let touch_right: bool = self.check_touch_right(&ball_x1, &ball_x2, &ball_y1, &ball_y2, &obj_x1, &obj_y1, &obj_y2);

        let touch_left: bool = self.check_touch_left(ball_x1, ball_x2, ball_y1, ball_y2, obj_x2, obj_y1, obj_y2);

        let touch_up: bool = self.check_touch_up(ball_x1, ball_x2, ball_y1, ball_y2, obj_x1, obj_x2, obj_y1, obj_y2);

        let touch_down: bool = self.check_touch_down(ball_x1, ball_x2, ball_y1, ball_y2, obj_x1, obj_x2, obj_y1, obj_y2);
        
        if touch_right || touch_left {
            self.reverse_x();
            self.increase_acceleration_x();
            return true;
        } else if touch_down || touch_up {
            self.reverse_y();
            self.increase_acceleration_y();
            return true;
        }

        false
    }

    pub fn randomize_angle() -> f64 {
        rand::thread_rng().gen_range(-60.0..60.0)
    }
}
