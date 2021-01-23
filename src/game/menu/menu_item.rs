use std::error::Error;

const MENU_ITEM_WIDTH: f64 = 200.0;
const MENU_ITEM_HEIGHT: f64 = 30.0;
const MENU_ITEM_POSITION_X: f64 = 0.0;
const MENU_ITEM_TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const MENU_ITEM_BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const MENU_ITEM_MARGIN: f64 = 15.0;

pub struct MenuItem {
    pub index: u8,
    pub text: String,
    pub text_color: [f32; 4],
    pub background_color: [f32; 4],
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
    pub height: f64
}

impl MenuItem {
    pub fn new(
        index: u8,
        text: String,
    ) -> Result<MenuItem, Box<dyn Error>> {
        let position_y: f64 = index as f64 * ((MENU_ITEM_HEIGHT + MENU_ITEM_MARGIN) * 2.0); 

        Ok(
            MenuItem {
                index,
                text,
                text_color: MENU_ITEM_TEXT_COLOR,
                background_color: MENU_ITEM_BACKGROUND_COLOR,
                position_x: MENU_ITEM_POSITION_X,
                position_y,
                width: MENU_ITEM_WIDTH,
                height: MENU_ITEM_HEIGHT
            }
        )
    }
}