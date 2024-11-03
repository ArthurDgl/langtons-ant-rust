use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const BLACK: u32 = 0;
const WHITE: u32 = (255 << 16) | (255 << 8) | 255;

const STEPS_PER_FRAME: usize = 1000;

struct Ant{
    x: i32,
    y: i32,

    dx: i8,
    dy: i8,
}

impl Ant {
    fn start() -> Ant {
        Ant {x: (WIDTH as i32)/2, y: (HEIGHT as i32)/2, dx: 0, dy: 1}
    }

    fn advance(&mut self) {
        self.x = self.x + self.dx as i32;
        self.x = self.x.rem_euclid(WIDTH as i32);
        self.y = self.y + self.dy as i32;
        self.y = self.y.rem_euclid(HEIGHT as i32);
    }

    fn turn_left(&mut self) {
        let temp: i8 = self.dx;
        self.dx = self.dy;
        self.dy = -temp;
    }

    fn turn_right(&mut self) {
        let temp: i8 = self.dx;
        self.dx = -self.dy;
        self.dy = temp;
    }

    fn turn(&mut self, buffer: &mut Vec<u32>) {
        let index = (self.y * WIDTH as i32 + self.x) as usize;
        let mut color = buffer[index];

        if color == BLACK {
            self.turn_left();
            color = WHITE;
        }
        else {
            self.turn_right();
            color = BLACK;
        }

        buffer[index] = color;
    }

    fn step(&mut self, buffer: &mut Vec<u32>) {
        self.turn(buffer);
        self.advance();
    }
}

fn main() {
    let mut window: Window = Window::new(
        "Langton's Ant",
        WIDTH as usize,
        HEIGHT as usize,
        WindowOptions::default()
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });

    window.limit_update_rate(Some(Duration::from_millis(10)));

    let mut buffer: Vec<u32> = vec![0; WIDTH as usize * HEIGHT as usize];

    // Initialization

    background_uniform(&mut buffer, 255);

    let mut ant = Ant::start();

    // Updates

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _ in 0..STEPS_PER_FRAME {
            ant.step(&mut buffer);
        }

        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap()
    }
}

fn background_uniform(buffer: &mut Vec<u32>, color: u32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            buffer[(y * WIDTH + x) as usize] = (color << 16) | (color << 8) | color;
        }
    }
}