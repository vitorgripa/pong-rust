use std::error::Error;

const WALL_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Wall {
    pub position_y: f64,
    pub position_x: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub color: [f32; 4]
}

impl Wall {
    pub fn new(width: f64, height: f64, position_x: f64, position_y: f64, rotation: f64) -> Result<Wall, Box<dyn Error>> {
        let color = WALL_COLOR;

        Ok(
            Wall {
                color,
                width,
                height,
                position_x,
                position_y,
                rotation
            }
        )

    }
}