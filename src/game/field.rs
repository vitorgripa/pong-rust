use std::error::Error;

mod wall;

use wall::Wall;

const FIELD_BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Field {
    pub external_walls: [Wall; 4],
    pub background_color: [f32; 4]
}

impl Field {
    pub fn new(window_width: f64, window_height: f64, window_margin: f64) -> Result<Field, Box<dyn Error>> {
        let external_walls = Field::create_external_walls(window_width, window_height, window_margin)?;

        let background_color = FIELD_BACKGROUND_COLOR;

        Ok (
            Field {
                external_walls,
                background_color
            }
        )
    }

    pub fn create_external_walls(
        window_width: f64,
        window_height: f64,
        window_margin: f64
    ) -> Result<[Wall; 4], Box<dyn Error>> {

        let wall1 = Wall::new(
            window_width - window_margin * 2.0,
            5.0,
            -(window_width / 2.0) + window_margin,
            -(window_height) / 2.0 + window_margin,
            0.0
        )?;

        let wall2 = Wall::new(
            5.0,
            window_height - window_margin * 2.0,
            300.0 - window_margin,
            -200.0 + window_margin,
            90.0
        )?;

        let wall3 = Wall::new(
            window_width - window_margin * 2.0,
            5.0,
            -(window_width / 2.0) + window_margin,
            200.0 - window_margin - 5.0,
            0.0
        )?;

        let wall4 = Wall::new(
            5.0,
            window_height - window_margin * 2.0,
            -300.0 + window_margin,
            -200.0 + window_margin,
            90.0
        )?;

        Ok([
            wall1,
            wall2,
            wall3,
            wall4
        ])
    }
}
