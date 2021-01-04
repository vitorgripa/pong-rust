extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

mod game;

use game::Game;

use piston::{Button, ButtonState};
use piston::event_loop::{Events, EventSettings};
use piston::input::{UpdateEvent, ButtonEvent, Key};

use piston_window::{PistonWindow, WindowSettings};

use opengl_graphics::OpenGL;

use graphics::{Context, Graphics, Transformed, clear, line, rectangle, text};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

const WINDOW_MARGIN: f64 = 15.0;

const WINDOW_WIDTH: f64 = 600.0;
const WINDOW_HEIGHT: f64 = 400.0;

const TRANSFORM_CENTER_SCREEN_X: f64 = WINDOW_WIDTH / 2.0;
const TRANSFORM_CENTER_SCREEN_Y: f64 = WINDOW_HEIGHT / 2.0;

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
        window.draw_2d(&e, |context, graphics, device| {
            clear(BLACK, graphics);
            
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

            game.field.external_walls.iter().for_each(|wall| rectangle(wall.color, [wall.position_x, wall.position_y, wall.width, wall.height], center_screen_transform, graphics));

            game.players.iter().for_each(|player| rectangle(player.color, [player.position_x, player.position_y, player.width, player.height], center_screen_transform, graphics));

            rectangle([1.0, 1.0, 1.0, 0.1], [-1.0, 0.0 - WINDOW_HEIGHT / 2.0 + WINDOW_MARGIN, 2.0, WINDOW_HEIGHT - WINDOW_MARGIN * 2.0], center_screen_transform, graphics);

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