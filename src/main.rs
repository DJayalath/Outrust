extern crate sdl2;
extern crate glm;

use glm::*;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use std::time::SystemTime;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;
const TITLE: &str = "Outrust";

const ROWS: u32 = 10;
const COLS: u32 = 10;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

fn main() {

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
 
    let window = video_subsystem.window(TITLE, WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("Problem creating window");

    // Build canvas
    let mut canvas = window.into_canvas().build().expect("Problem building canvas from window");
    let texture_creator = canvas.texture_creator();

    let mut font = ttf_context.load_font("./RobotoMono-Regular.ttf", 128).unwrap();
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut score: u32 = 0;

    sdl_context.mouse().show_cursor(false);
    sdl_context.mouse().capture(true);
    sdl_context.mouse().set_relative_mouse_mode(true);

    let mut event_pump = sdl_context.event_pump().expect("Problem creating event pump");
    let mut timer = Timer::new(0, SystemTime::now(), SystemTime::now());

    let mut bumper = Bumper::new(ivec2(WIDTH as i32 / 2, HEIGHT as i32 - 10), 0.05, uvec2(100, 10));
    let mut ball = Ball::new(ivec2(WIDTH as i32 / 2, HEIGHT as i32 - 20), dvec2(0.5, -0.5), 10);

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
        ball.update(timer.frame_time, &mut score);
        bumper.update(timer.frame_time);

        while bbumper_collision(&bumper, &mut ball) {

            ball.pos.x -= bb_dir(ball.vel.x);
            if !bbumper_collision(&bumper, &mut ball) {
                if ball.vel.x < 0.0 {
                    ball.displace(Side::Left);
                } else {
                    ball.displace(Side::Right);
                }

                break
            }

            ball.pos.y -= bb_dir(ball.vel.y);
            if !bbumper_collision(&bumper, &mut ball) {
                if ball.vel.y < 0.0 {
                    ball.displace(Side::Top);
                } else {
                    ball.displace(Side::Bottom);
                }

                break
            }
        }

        for j in 0..ROWS {
            for i in 0..COLS {

                if bricks[i as usize + COLS as usize * j as usize].active {
                    while bb_collision(&mut bricks[i as usize + COLS as usize * j as usize], &mut ball, &mut score) {

                        ball.pos.x -= bb_dir(ball.vel.x) as i32;
                        if !bb_collision(&mut bricks[i as usize + COLS as usize * j as usize], &mut ball, &mut score) {
                            if ball.vel.x < 0.0 {
                                ball.displace(Side::Left);
                            } else {
                                ball.displace(Side::Right);
                            }

                            bricks[i as usize + COLS as usize * j as usize].active = false;
                            score += 1;

                            break
                        }

                        ball.pos.y -= bb_dir(ball.vel.y) as i32;
                        if !bb_collision(&mut bricks[i as usize + COLS as usize * j as usize], &mut ball, &mut score) {
                            if ball.vel.y < 0.0 {
                                ball.displace(Side::Top);
                            } else {
                                ball.displace(Side::Bottom);
                            }

                            bricks[i as usize + COLS as usize * j as usize].active = false;
                            score += 1;

                            break
                        }


                    }
                    // ball_brick_collision(&mut bricks[i as usize + COLS as usize * j as usize], &mut ball, &mut score);
                    bricks[i as usize + COLS as usize * j as usize].draw(&mut canvas);
                }
            }
        }

        let surface = font.render(&format!("Score: {}", score)[..]).blended(Color::RGB(0xFF, 0xFF, 0xFF)).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
        let TextureQuery { width, height, .. } = texture.query();
        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 670;
        let target = get_rect(width, height, WIDTH as u32 - padding, HEIGHT as u32 - padding, 20, 20);

        ball.draw(&mut canvas);
        bumper.draw(&mut canvas);
        canvas.copy(&texture, None, Some(target)).unwrap();

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

    fn update(&mut self, frame_time: u128, score: &mut u32) {
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
            *score = 0;
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

fn bbumper_collision(bumper: &Bumper, ball: &mut Ball) -> bool {

    // Collision x-axis?
    let collision_x: bool = bumper.pos.x + bumper.size.x as i32 >= ball.pos.x &&
        ball.pos.x + ball.size as i32 >= bumper.pos.x;

    // Collision y-axis?
    let collision_y: bool = bumper.pos.y + bumper.size.y as i32 >= ball.pos.y &&
        ball.pos.y + ball.size as i32 >= bumper.pos.y;

    collision_x && collision_y
}

fn bb_collision(brick: &mut Brick, ball: &mut Ball, score: &mut u32) -> bool {

    // Collision x-axis?
    let collision_x: bool = brick.pos.x + brick.size.x as i32 >= ball.pos.x &&
        ball.pos.x + ball.size as i32 >= brick.pos.x;

    // Collision y-axis?
    let collision_y: bool = brick.pos.y + brick.size.y as i32 >= ball.pos.y &&
        ball.pos.y + ball.size as i32 >= brick.pos.y;

    collision_x && collision_y
}

fn bb_dir(vel: f64) -> i32 {
    if vel > 0.0 {
        1
    } else if vel < 0.0 {
        -1
    } else {
        0
    }
}

// Scale fonts to a reasonable size when they're too big (though they might look less smooth)
fn get_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32, x: u32, y: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    rect!(x, y, w, h)
}