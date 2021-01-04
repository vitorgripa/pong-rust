pub struct Wall {
    pub position_y: f64,
    pub position_x: f64,
    pub width: f64,
    pub height: f64,
    pub rotation: f64,
    pub color: [f32; 4]
}

impl Wall {
    pub fn new(color: [f32; 4], width: f64, height: f64, position_x: f64, position_y: f64, rotation: f64) -> Self {
        Self {
            color,
            width,
            height,
            position_x,
            position_y,
            rotation
        }
    }
}
