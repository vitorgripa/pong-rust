mod wall;

use wall::Wall;

pub struct Field {
    pub external_walls: [Wall; 4],
    pub background_color: [f32; 4]
}

impl Field {
    pub fn new(window_width: f64, window_height: f64, window_margin: f64) -> Self {
        Self {
            external_walls: Field::create_external_walls(window_width, window_height, window_margin),
            background_color: [0.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn create_external_walls(
        window_width: f64,
        window_height: f64,
        window_margin: f64
    ) -> [Wall; 4] {
        [
            Wall::new(
                [1.0, 1.0, 1.0, 1.0],
                window_width - window_margin * 2.0,
                5.0,
                -(window_width / 2.0) + window_margin,
                -(window_height) / 2.0 + window_margin,
                0.0
            ),
            Wall::new(
                [1.0, 1.0, 1.0, 1.0],
                5.0,
                window_height - window_margin * 2.0,
            300.0 - window_margin,
            -200.0 + window_margin,
            90.0
            ),
            Wall::new(
                [1.0, 1.0, 1.0, 1.0],
                window_width - window_margin * 2.0,
                5.0,
                -(window_width / 2.0) + window_margin,
                200.0 - window_margin - 5.0,
                0.0
            ),
            Wall::new(
                [1.0, 1.0, 1.0, 1.0],
                5.0,
                window_height - window_margin * 2.0,
                -300.0 + window_margin,
                -200.0 + window_margin,
                90.0
            )
        ]
    }
}
