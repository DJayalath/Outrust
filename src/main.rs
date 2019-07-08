extern crate sdl2;
extern crate glm;

use glm::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::SystemTime;
// use std::collections::HashSet;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const TITLE: &str = "Outrust";

const BUMPER_ACCEL: f64 = 0.05;
const BUMPER_MAX: f64 = 1.1;
const BUMPER_BOOST: f64 = 10.0;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window(TITLE, WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Problem creating window");

    // Build canvas
    let mut canvas = window.into_canvas().build().expect("Problem building canvas from window");

    let mut event_pump = sdl_context.event_pump().expect("Problem creating event pump");
    let mut timer = Timer::new(0, SystemTime::now(), SystemTime::now());

    let mut bumper = Bumper::new(ivec2(WIDTH as i32 / 2, HEIGHT as i32 - 30), 0.05, uvec2(100, 10));
    let mut ball = Ball::new(ivec2(50, 50), dvec2(0.5, 1.1), 10);

   'running: loop {

        // Reset buffer
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Event polling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        // Create a set of pressed Keys.
        let keys: std::collections::HashSet<sdl2::keyboard::Keycode> = 
            event_pump.keyboard_state().pressed_scancodes().filter_map(Keycode::from_scancode).collect();

        if keys.contains(&Keycode::Left) {
            bumper.displace(Move::Left);
        } else if keys.contains(&Keycode::Right) {
            bumper.displace(Move::Right);
        }

        // The rest of the game loop goes here...
        ball.update(timer.frame_time);
        bumper.update(timer.frame_time);

        ball.draw(&mut canvas);
        bumper.draw(&mut canvas);

        canvas.present();

        timer.update();
    }
}

enum Move {
    Left,
    Right,
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

struct Ball {
    pos: Vector2<i32>,
    vel: Vector2<f64>,
    size: u32,
}

impl Ball {
    fn new (pos: Vector2<i32>, vel: Vector2<f64>, size: u32) -> Ball {
        Ball { pos, vel, size }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, self.size, self.size))
            .expect("Problem drawing ball");
    }

    fn update(&mut self, frame_time: u128) {
        self.pos.x += (self.vel.x * frame_time as f64) as i32;
        self.pos.y += (self.vel.y * frame_time as f64) as i32;

        if self.pos.x < 0 {
            self.displace(Side::Left)
        } else if self.pos.y < 0 {
            self.displace(Side::Top)
        } else if self.pos.x + self.size as i32 > WIDTH as i32 - 1 {
            self.displace(Side::Right)
        } else if self.pos.y + self.size as i32 > HEIGHT as i32 - 1 {
            self.displace(Side::Bottom)
        }
    }

    fn displace(&mut self, side: Side) {
        self.vel = match side {
            Side::Top => reflect(self.vel, dvec2(0.0, 1.0)),
            Side::Bottom => reflect(self.vel, dvec2(0.0, 1.0)),
            Side::Left => reflect(self.vel, dvec2(1.0, 0.0)),
            Side::Right => reflect(self.vel, dvec2(1.0, 0.0)),
        }
    }
}

struct Bumper {
    pos: Vector2<i32>,
    vel: f64,
    size: Vector2<u32>,
}

impl Bumper {
    fn new (pos: Vector2<i32>, vel: f64, size: Vector2<u32>) -> Bumper {
        Bumper { pos, vel, size }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y))
            .expect("Problem drawing bumper");
    }

    fn displace(&mut self, dir: Move) {
        self.vel += match dir {
            Move::Left => {
                if sign(-BUMPER_ACCEL) != sign(self.vel) {
                    -BUMPER_ACCEL * BUMPER_BOOST
                }
                else {
                    -BUMPER_ACCEL
                }
            },
            Move::Right => {
                if sign(BUMPER_ACCEL) != sign(self.vel) {
                    BUMPER_ACCEL * BUMPER_BOOST
                }
                else {
                    BUMPER_ACCEL
                }
            },
        };

        if self.vel > BUMPER_MAX {
            self.vel = BUMPER_MAX;
        } else if self.vel < -BUMPER_MAX {
            self.vel = -BUMPER_MAX
        }
    }

    fn update(&mut self, frame_time: u128) {
        self.pos.x += (self.vel * frame_time as f64) as i32;

        // Test walls
        if self.pos.x < 0 {
            self.pos.x = 0;
            self.vel = 0.0;
        } else if self.pos.x + self.size.x as i32 > WIDTH as i32 - 1 {
            self.pos.x = WIDTH as i32 - 1 - self.size.x as i32;
            self.vel = 0.0;
        }
    }
}

struct Timer {
    frame_time: u128,
    now: std::time::SystemTime,
    last: std::time::SystemTime,
}

impl Timer {
    fn new(frame_time: u128, now: std::time::SystemTime, last: std::time::SystemTime) -> Timer {
        Timer { frame_time, now, last }
    }

    fn update(&mut self) {
        self.now = SystemTime::now();

        self.frame_time = self.now.duration_since(self.last)
            .expect("Problem updating time").as_millis();

        self.last = self.now;
    }
}