use std::error::Error;

mod menu_item;

use menu_item::MenuItem;

pub struct Menu {
    pub items: [MenuItem; 3],
    pub selected_item: u8,
}

impl Menu {
    pub fn new() -> Result<Menu, Box<dyn Error>> {
        let menu_item1: MenuItem = MenuItem::new(
            1,
            String::from("Retomar"),
        )?;

        let menu_item2: MenuItem = MenuItem::new(
            2,
            String::from("Opcões")
        )?;

        let menu_item3: MenuItem = MenuItem::new(
            3,
            String::from("Sair")
        )?;

        let items: [MenuItem; 3] = [
            menu_item1,
            menu_item2,
            menu_item3
        ];

        let selected_item: u8 = 0;

        Ok (
            Menu {
                selected_item,
                items
            }
        )
    }
}
