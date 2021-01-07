use piston::{Button};
use piston::input::{Key};

mod player;
mod ball;
mod field;

use player::Player;
use ball::Ball;
use field::Field;

const WINDOW_MARGIN: f64 = 15.0;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 400.0;

pub struct Game {
    pub players: [Player; 2],
    pub ball: Ball,
    pub field: Field,
    pub state: i32,
}

impl Game {
    pub fn init(&mut self) {
        for player in self.players.iter_mut() {
            player.reset_position();
            player.reset_score();
        }

        self.ball.reset_position();
        self.ball.reset_acceleration();

        self.start();
    }

    pub fn new() -> Self {
        let players: [Player; 2] = [
            Player::new(250.0, 1),
            Player::new(-250.0, 2)
        ];

        Self {
            ball: Ball::new(),
            field: Field::new(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_MARGIN),
            players,
            state: 1
        }
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

    pub fn pause(&mut self) {
        self.change_game_state(1);
    }

    pub fn start(&mut self) {
        self.change_game_state(0);
        self.ball.randomize_angle();
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
                    self.init();
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
                }
            },
            &Button::Keyboard(Key::Down) => {
                if self.is_playing() {
                    if self.players[0].position_y < 175.0 {
                        self.players[0].move_down();
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
            &Button::Keyboard(Key::R) => {
                self.init();
            },
            &Button::Keyboard(Key::Space) => {
                if self.is_playing() {
                    self.pause();
                } else {
                    self.start();
                }
            },
            &Button::Keyboard(Key::K) => {
                self.ball.randomize_angle();
            }
            _ => {}
        }
    }
}