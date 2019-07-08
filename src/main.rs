extern crate sdl2;
extern crate glm;

use glm::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::SystemTime;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const TITLE: &str = "Outrust";

const ROWS: u32 = 10;
const COLS: u32 = 10;

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window(TITLE, WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Problem creating window");

    // Build canvas
    let mut canvas = window.into_canvas().build().expect("Problem building canvas from window");

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut event_pump = sdl_context.event_pump().expect("Problem creating event pump");
    let mut timer = Timer::new(0, SystemTime::now(), SystemTime::now());

    let mut bumper = Bumper::new(ivec2(WIDTH as i32 / 2, HEIGHT as i32 - 10), 0.05, uvec2(100, 10));
    let mut ball = Ball::new(ivec2(50, 50), dvec2(1.1, 1.1), 10);

    let mut bricks: Vec<Brick> = Vec::with_capacity(100);

    // Total width: 690, height: 290
    for j in 0..ROWS {
        for i in 0..COLS {
            bricks.push(Brick::new(ivec2(300 + i as i32 * 60 + i as i32 * 10, 145 + 20 * j as i32 + 10 * j as i32), uvec2(60, 20), true));
        }
    }

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

        // Update bumper position
        bumper.pos.x = event_pump.mouse_state().x();

        // The rest of the game loop goes here...
        ball.update(timer.frame_time);
        bumper.update(timer.frame_time);
        ball_bumper_collision(&bumper, &mut ball);

        for j in 0..ROWS {
            for i in 0..COLS {
                ball_brick_collision(&mut bricks[i as usize + COLS as usize * j as usize], &mut ball);
                bricks[i as usize + COLS as usize * j as usize].draw(&mut canvas);
            }
        }

        ball.draw(&mut canvas);
        bumper.draw(&mut canvas);

        canvas.present();

        timer.update();
    }
}

enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

struct Brick {
    pos: Vector2<i32>,
    size: Vector2<u32>,
    active: bool,
}

impl Brick {
    fn new (pos: Vector2<i32>, size: Vector2<u32>, active: bool) -> Brick {
        Brick { pos, size, active }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        if self.active {
            canvas.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
            canvas.fill_rect(Rect::new(self.pos.x, self.pos.y, self.size.x, self.size.y))
                .expect("Problem drawing ball");
        }
    }
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

        // Test wall collision
        if self.pos.x < 0 {
            self.pos.x = 0;
            self.displace(Side::Left)
        } else if self.pos.y < 0 {
            self.pos.y = 0;
            self.displace(Side::Top)
        } else if self.pos.x + self.size as i32 > WIDTH as i32 - 1 {
            self.pos.x = WIDTH as i32 - 1 - self.size as i32;
            self.displace(Side::Right)
        } else if self.pos.y + self.size as i32 > HEIGHT as i32 - 1 {
            self.pos.y = HEIGHT as i32 - 1 - self.size as i32;
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

fn ball_bumper_collision(bumper: &Bumper, ball: &mut Ball) {
    if ball.pos.y >= bumper.pos.y {
        if ball.pos.x >= bumper.pos.x && ball.pos.x <= bumper.pos.x + bumper.size.x as i32 {
            ball.pos.y = bumper.pos.y;
            ball.displace(Side::Bottom);
        } else if ball.pos.x + ball.size as i32 >= bumper.pos.x && 
            ball.pos.x + ball.size as i32 <= bumper.pos.x + bumper.size.x as i32 {
            ball.pos.y = bumper.pos.y;
            ball.displace(Side::Bottom);
        }
    }
}

fn ball_brick_collision(brick: &mut Brick, ball: &mut Ball) {

    if brick.active {

        if ball.pos.y >= brick.pos.y && ball.pos.y <= brick.pos.y + brick.size.y as i32 {

            if ball.pos.x >= brick.pos.x && ball.pos.x <= brick.pos.x + brick.size.x as i32 {
                brick.active = false;
                if ball.vel.y < 0.0 {
                    ball.displace(Side::Top)
                } else {
                    ball.displace(Side::Bottom)
                }
            } else if ball.pos.x + ball.size as i32 >= brick.pos.x && ball.pos.x + ball.size as i32 <= brick.pos.x + brick.size.x as i32 {
                brick.active = false;
                if ball.vel.y < 0.0 {
                    ball.displace(Side::Top)
                } else {
                    ball.displace(Side::Bottom)
                }
            }

        } else if ball.pos.y + ball.size as i32 >= brick.pos.y && ball.pos.y + ball.size as i32 <= brick.pos.y + brick.size.y as i32 {

            if ball.pos.x >= brick.pos.x && ball.pos.x <= brick.pos.x + brick.size.x as i32 {
                brick.active = false;
                if ball.vel.y < 0.0 {
                    ball.displace(Side::Top)
                } else {
                    ball.displace(Side::Bottom)
                }
            } else if ball.pos.x + ball.size as i32 >= brick.pos.x && ball.pos.x + ball.size as i32 <= brick.pos.x + brick.size.x as i32 {
                brick.active = false;
                if ball.vel.y < 0.0 {
                    ball.displace(Side::Top)
                } else {
                    ball.displace(Side::Bottom)
                }
            }

        }
    }
    // Top side

    // Bottom side

    // Left side

    // Right side

}