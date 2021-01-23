extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use serde::{Serialize, Deserialize};
use opengl_graphics::OpenGL;
use piston::{Button, ButtonState, EventSettings, Events, Size, WindowSettings};
use piston::input::{Key};

mod player;
mod ball;
mod field;
mod menu;

use piston_window::PistonWindow;
use player::Player;
use ball::Ball;
use field::Field;
use menu::Menu;
use graphics::*;
use piston::input::{UpdateEvent, ButtonEvent};

const CENTER_SCREEN_TRANSFORM: [[f64; 3]; 2] = [[0.0033333333333333335, 0.0, 0.0], [0.0, -0.005, 0.0]];
const GRAPHICS_API: OpenGL = OpenGL::V3_2;
const WINDOW_MARGIN: f64 = 15.0;
const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 400.0;
const SEPARATOR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 0.1];
const WINDOW_SIZES: [[f64; 2]; 9] = [
    [320.0, 240.0],
    [640.0, 360.0],
    [800.0, 600.0],
    [1024.0, 768.0],
    [1280.0, 720.0],
    [1366.0, 768.0],
    [1440.0, 900.0],
    [1600.0, 900.0],
    [1920.0, 1080.0]
];

#[derive(Debug, Serialize, Deserialize)]
enum GameDificulty {
    Easy,
    Medium,
    Hard
}

#[derive(Debug, Serialize, Deserialize)]
struct SoundOptions {
    effects_volume: f64,
    music_volume: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphicsOptions{
    window_size: [f64; 2],
    fullscreen: bool,
    vsync: bool
}

#[derive(Debug, Serialize, Deserialize)]
struct GameOptions {
    dificulty: GameDificulty
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    sound_options: SoundOptions,
    graphics_options: GraphicsOptions,
    game_options: GameOptions
}

impl Settings {
    pub fn new() -> Result<Settings, Box<dyn Error>> {
        let settings_file: File = File::open("settings.json")?;
        let settings_reader: BufReader<File> = BufReader::new(settings_file);

        let settings: Settings = serde_json::from_reader(settings_reader)?;

        Ok(settings)
    }

}
pub struct Game {
    pub players: [Player; 2],
    pub ball: Ball,
    pub field: Field,
    pub state: i32,
    pub menu: Menu,
    pub window: PistonWindow,
    pub settings: Settings
}

impl Game {
    pub fn new() -> Result<Game, Box<dyn Error>> {
        let settings: Settings = Settings::new()?;

        let title: String = String::from("PONG");
        let window_size: Size = Size::from(settings.graphics_options.window_size);
        let samples: u8 = 0;
        let fullscreen: bool = settings.graphics_options.fullscreen;
        let exit_on_esc: bool = true;
        let automatic_close: bool = true;
        let vsync: bool = settings.graphics_options.vsync;
        let srgb: bool = true;
        let resizable: bool = false;
        let decorated: bool = false;
        let controllers: bool = true;
        let transparent: bool = true;

        let window_settings: WindowSettings = WindowSettings::new(
            title,
            window_size
        )
            .graphics_api(GRAPHICS_API)
            .exit_on_esc(exit_on_esc)
            .automatic_close(automatic_close)
            .vsync(vsync)
            .srgb(srgb)
            .resizable(resizable)
            .decorated(decorated)
            .controllers(controllers)
            .transparent(transparent)
            .fullscreen(fullscreen)
            .samples(samples);

        let window: PistonWindow = window_settings.build()?;

        let player1: Player = Player::new(250.0, 1)?;
        let player2: Player = Player::new(-250.0, 2)?; 

        let players: [Player; 2] = [
            player1,
            player2
        ];

        let ball: Ball = Ball::new()?;

        let field: Field = Field::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_MARGIN)?;

        let menu: Menu = Menu::new()?;

        let state: i32 = 0;
        
        Ok(
            Game {
                ball,
                field,
                players,
                state,
                menu,
                window,
                settings
            }
        )
    }

    pub fn change_game_state(&mut self, state: i32) {
        self.state = state;
    }

    pub fn is_playing(&self) -> bool{
        match self.state {
            0 => true,
            _ => false
        }
    }

    pub fn start(&mut self) {
        self.change_game_state(0);
    }

    pub fn pause(&mut self) {
        self.change_game_state(1);
    }

    pub fn lose(&mut self) {
        self.change_game_state(2);
    }

    pub fn check_lose_game(&self) -> bool{
        if self.ball.position_x > self.players[0].position_x + self.players[0].width || self.ball.position_x + self.ball.width < self.players[1].position_x {
            return true
        }

        false
    }

    pub fn update(&mut self){
        match self.state {
            0 => {
                for player in self.players.iter_mut() {
                    let player_collision: bool = self.ball.check_collision(
                        &player.position_x,
                        &(player.position_x + player.width),
                        &player.position_y,
                        &(player.position_y + player.height)
                    );

                    if player_collision {
                        player.make_point();
                    }
                };


                for wall in self.field.external_walls.iter() {
                    self.ball.check_collision(
                        &wall.position_x,
                        &(wall.position_x + wall.width),
                        &wall.position_y,
                        &(wall.position_y + wall.height)
                    );
                }
        
                let game_lose: bool = self.check_lose_game();
        
                if game_lose {
                    self.lose()
                }

                self.ball.position_y += self.ball.acceleration_y;
                self.ball.position_x += self.ball.acceleration_x;
            }
            _ => {
            }

        }
    }

    pub fn key_pressed(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) => {
                if self.is_playing() {
                    if self.players[0].position_y > -175.0 {
                        self.players[0].move_up();
                    }
                } else {
                    if self.menu.selected_item == 0 {
                        self.menu.selected_item = 2;
                    } else {
                        self.menu.selected_item -= 1;
                    }
                }
            },
            &Button::Keyboard(Key::Down) => {
                if self.is_playing() {
                    if self.players[0].position_y < 175.0 {
                        self.players[0].move_down();
                    }
                } else {
                    if self.menu.selected_item == 2 {
                        self.menu.selected_item = 0;
                    } else {
                        self.menu.selected_item += 1;
                    }
                }
            },
            &Button::Keyboard(Key::W) => {
                if self.is_playing() {
                    if self.players[1].position_y > -175.0 {
                        self.players[1].move_up();
                    }
                }
            },
            &Button::Keyboard(Key::S) => {
                if self.is_playing() {
                    if self.players[1].position_y < 175.0 {
                        self.players[1].move_down();
                    }
                }
            },
            &Button::Keyboard(Key::Space) => {
                if self.is_playing() {
                    self.pause();
                } else {
                    self.start();
                }
            }
            _ => {}
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let separator_rect: [f64; 4] = [-1.0, 0.0 - WINDOW_HEIGHT / 2.0 + WINDOW_MARGIN, 2.0, WINDOW_HEIGHT - WINDOW_MARGIN * 2.0];
        let mut events: Events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut self.window) {
            let field = &self.field;
            let external_walls = &field.external_walls;
            let players  = &self.players;
            let ball = &self.ball;

            self.window.draw_2d(&e, |_context, graphics, _device| {
                clear(field.background_color, graphics);

                external_walls.iter().for_each(|wall| {
                    rectangle(wall.color, [wall.position_x, wall.position_y, wall.width, wall.height], CENTER_SCREEN_TRANSFORM , graphics);
                });

                players.iter().for_each(|player| {
                    rectangle(player.color, [player.position_x, player.position_y, player.width, player.height], CENTER_SCREEN_TRANSFORM, graphics);
                });

                rectangle(ball.color, [ball.position_x, ball.position_y, ball.width, ball.height], CENTER_SCREEN_TRANSFORM, graphics);

                rectangle(SEPARATOR_COLOR, separator_rect, CENTER_SCREEN_TRANSFORM, graphics);

            });

            if let Some(_args) = e.update_args() {
                    self.update();

                    if self.state == 2 {
                        break;
                    }
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    self.key_pressed(&k.button);
                }
            }
        }

        Ok(())
    }
}