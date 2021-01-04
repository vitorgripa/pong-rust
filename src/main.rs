extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::{Button, ButtonState};
use piston::event_loop::{Events, EventSettings};
use piston::input::{UpdateEvent, ButtonEvent, Key};

use piston_window::{PistonWindow, WindowSettings};

use opengl_graphics::OpenGL;

use graphics::{clear, rectangle, line, text, Transformed, Context};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

// const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
// const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
// const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

const MARGIN: f64 = 15.0;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 400.0;

const PLAYER_WIDTH: f64 = 5.0;
const PLAYER_HEIGHT: f64 = 60.0;

const BALL_HEIGHT: f64 = 10.0;
const BALL_WIDTH: f64 = 10.0;

const TRANSFORM_CENTER_SCREEN_X: f64 = WINDOW_WIDTH / 2.0;
const TRANSFORM_CENTER_SCREEN_Y: f64 = WINDOW_HEIGHT / 2.0;

pub struct Player {
    number: u8,
    position_y: f64,
    position_x: f64,
    width: f64,
    height: f64,
    color: [f32; 4],
    score: u32,
}

impl Player {
    pub fn new(position_x: f64, number: u8) -> Self {
        Self {
            position_y: 0.0 - PLAYER_HEIGHT / 2.0,
            position_x,
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            color: WHITE,
            score: 0,
            number
        }
    }

    fn make_point(&mut self) {
        self.score += 1;
    }

    fn reset_score(&mut self) {
        self.score = 0;
    }

    fn move_up(&mut self) {
        self.position_y -= 5.0;
    }

    fn move_down(&mut self) {
        self.position_y += 5.0;
    }
}

pub struct Ball {
    position_y: f64,
    position_x: f64,
    width: f64,
    height: f64,
    acceleration_x: f64,
    acceleration_y: f64,
    color: [f32; 4]
}

impl Ball {

    fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            width: BALL_WIDTH,
            height: BALL_HEIGHT,
            acceleration_x: 1.0,
            acceleration_y: 1.0,
            color: WHITE
        }
    }

    fn check_touch(&mut self, obj_x1: &f64, obj_x2: &f64, obj_y1: &f64, obj_y2: &f64) -> bool {
        let ball_x1: &f64 = &self.position_x;
        let ball_x2: &f64 = &(self.position_x + self.width);
        let ball_y1: &f64 = &self.position_y;
        let ball_y2: &f64 = &(self.position_y + self.height);

        if ball_x1 > obj_x2 || ball_x2 < obj_x1  {
            return false;
        } else if ball_y1 > obj_y2 || ball_y2 < obj_y1 {
            return false;
        }
        true
    }
}

pub struct Wall {
    position_y: f64,
    position_x: f64,
    width: f64,
    height: f64,
    rotation: f64,
    color: [f32; 4]
}

pub struct Field {
    walls: [Wall; 4],
    background_color: [f32; 4]
}

impl Field {
    fn new() -> Field {
        Field {
            walls: Field::create_external_walls(),
            background_color: BLACK
        }
    }

    fn create_external_walls() -> [Wall; 4] {
        [
            Wall {
                color: WHITE,
                width: WINDOW_WIDTH - MARGIN * 2.0,
                height: 5.0,
                position_x: -(WINDOW_WIDTH / 2.0) + MARGIN,
                position_y: -(WINDOW_HEIGHT) / 2.0 + MARGIN,
                rotation: 0.0
            },
            Wall {
                color: WHITE,
                width: 5.0,
                height: WINDOW_HEIGHT - MARGIN * 2.0,
                position_x: 300.0 - MARGIN,
                position_y: -200.0 + MARGIN,
                rotation: 90.0
            },
            Wall {
                color: WHITE,
                width: WINDOW_WIDTH - MARGIN * 2.0,
                height: 5.0,
                position_x: -(WINDOW_WIDTH / 2.0) + MARGIN,
                position_y: 200.0 - MARGIN - 5.0,
                rotation: 0.0
            },
            Wall {
                color: WHITE,
                width: 5.0,
                height: WINDOW_HEIGHT - MARGIN * 2.0,
                position_x: -300.0 + MARGIN,
                position_y: -200.0 + MARGIN,
                rotation: 90.0
            }
        ]
    }
}

pub struct Game {
    players: [Player; 2],
    ball: Ball,
    field: Field,
    state: i32,
}

/* States {
    0: "playing",
    1: "paused",
}
*/

impl Game {
    fn init(&mut self) {
        for player in self.players.iter_mut() {
            player.position_y =  0.0 - player.height / 2.0;
            player.reset_score();
        }

        self.ball.position_y = 0.0;
        self.ball.position_x = 0.0;

        self.start();
    }

    fn new() -> Self {
        let players: [Player; 2] = [
            Player::new(250.0, 1),
            Player::new(-250.0, 2)
        ];

        Self {
            ball: Ball::new(),
            field: Field::new(),
            players,
            state: 0
        }
    }

    fn change_game_state(&mut self, state: i32) {
        self.state = state;
    }

    fn is_playing(&self) -> bool{
        match self.state {
            0 => true,
            _ => false
        }
    }

    fn pause(&mut self) {
        self.change_game_state(1);
    }

    fn start(&mut self) {
        self.change_game_state(0);
    }

    fn check_lose_game(&self) -> bool{
        if self.ball.position_x + self.ball.width > self.players[0].position_x || self.ball.position_x < self.players[1].position_x + self.players[1].width {
            return true
        }

        false
    }

    fn update(&mut self){
        match self.state {
            0 => {
                for player in self.players.iter_mut() {
                    let player_ball_touch: bool = self.ball.check_touch(
                        &player.position_x,
                        &(player.position_x + player.width),
                        &player.position_y,
                        &(player.position_y + player.height)
                    );

                    if player_ball_touch {
                        self.ball.acceleration_x *= -1.0;
                        player.make_point();
                    }
                }
        
                for wall in self.field.walls.iter() {
                    let wall_ball_touch = self.ball.check_touch(
                        &wall.position_x,
                        &(wall.position_x + wall.width),
                        &wall.position_y,
                        &(wall.position_y + wall.height)
                    );
        
                    if wall_ball_touch {
                        if wall.rotation == 0.0 {
                            self.ball.acceleration_y *= -1.0;
                        } else {
                            self.ball.acceleration_x *= -1.0;
                        }
                    }
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

    fn key_pressed(&mut self, btn: &Button) {
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
            }
            _ => {}
        }
    }
}

fn main() {
    let opengl: OpenGL = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new("Pong", [WINDOW_WIDTH, WINDOW_HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();
    
    let mut glyphs_cache = window.load_font("assets/fonts/Roboto-Medium.ttf").unwrap();

    let mut game = Game::new();

    game.init();

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {

        window.draw_2d(&e, |context: Context, graphics, device| {
            clear(game.field.background_color, graphics);

            let center_screen_transform = context
            .transform
            .trans(TRANSFORM_CENTER_SCREEN_X, TRANSFORM_CENTER_SCREEN_Y);

            let player1_score_transform  = context
                .transform
                .trans(TRANSFORM_CENTER_SCREEN_X, TRANSFORM_CENTER_SCREEN_Y)
                .trans(50.0, 8.0);
            
            let player2_score_transform = context
                .transform
                .trans(TRANSFORM_CENTER_SCREEN_X, TRANSFORM_CENTER_SCREEN_Y)
                .trans(-50.0, 8.0);

            game.field.walls.iter().for_each(|wall| rectangle(wall.color, [wall.position_x, wall.position_y, wall.width, wall.height], center_screen_transform, graphics));

            game.players.iter().for_each(|player| rectangle(player.color, [player.position_x, player.position_y, player.width, player.height], center_screen_transform, graphics));

            line(WHITE, 2.0, [2.0, -(WINDOW_HEIGHT / 2.0) + MARGIN, 2.0, (WINDOW_HEIGHT / 2.0) - MARGIN], center_screen_transform, graphics);

            text(WHITE, 16, &format!("{}", game.players[0].score), &mut glyphs_cache, player1_score_transform, graphics).unwrap();
            text(WHITE, 16, &format!("{}", game.players[1].score), &mut glyphs_cache, player2_score_transform, graphics).unwrap();

            rectangle(game.ball.color, [game.ball.position_x, game.ball.position_y, game.ball.width, game.ball.height], center_screen_transform, graphics);

            glyphs_cache.factory.encoder.flush(device);
        });

        if let Some(_args) = e.update_args() {
                game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.key_pressed(&k.button);
            }
        }
    }
} 