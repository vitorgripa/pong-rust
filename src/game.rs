use std::error::Error;
use opengl_graphics::OpenGL;
use piston::{Button, ButtonState, EventSettings, Events, WindowSettings};
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

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Game {
    pub players: [Player; 2],
    pub ball: Ball,
    pub field: Field,
    pub state: i32,
    pub menu: Menu,
    pub window: PistonWindow
}

impl Game {
    pub fn new() -> Result<Game, Box<dyn Error>> {
        let window: PistonWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
            .graphics_api(GRAPHICS_API)
            .exit_on_esc(true)
            .resizable(false)
            .build()?;

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
                window
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
                    rectangle(WHITE, [wall.position_x, wall.position_y, wall.width, wall.height], CENTER_SCREEN_TRANSFORM , graphics);
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