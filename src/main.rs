use macroquad::prelude::*;
use macroquad::rand::{gen_range, srand};
use std::time::{SystemTime, UNIX_EPOCH};

const SCREEN_WIDTH: i32 = 600;
const SCREEN_HEIGHT: i32 = 600;
const BLOCK_SIZE: i32 = 20;

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct SnakeBlock {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Snake {
    blocks: Vec<SnakeBlock>,
    direction: Direction,
    grow: bool,
}

#[derive(Debug)]
struct Apple {
    x: i32,
    y: i32,
}

impl Snake {
    fn new() -> Snake {
        let mut blocks: Vec<SnakeBlock> = Vec::new();
        blocks.push(SnakeBlock { x: 20, y: 20 });
        blocks.push(SnakeBlock { x: 19, y: 20 });
        Snake {
            blocks,
            direction: Direction::Up,
            grow: false,
        }
    }

    fn slither(&mut self) {
        /* Create a new block, then drop the last block */
        let dir = match self.direction {
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Right => (1, 0),
        };
        self.blocks.insert(
            0,
            SnakeBlock {
                x: self.blocks[0].x + dir.0,
                y: self.blocks[0].y + dir.1,
            },
        );
        if !self.grow {
            self.blocks.pop();
        }
        self.grow = false;
    }
    fn grow(&mut self) {
        self.grow = true;
    }
    fn is_dead(&self) -> bool {
        let h = &self.blocks[0];
        let mut body: bool = false;
        let mut screen: bool = is_out_of_bounds(h.x, h.y);
        let mut iter = self.blocks.iter();
        iter.next();

        for b in iter {
            if b.x == h.x && b.y == h.y {
                body = true;
            }
            if is_out_of_bounds(b.x, b.y) {
                screen = true;
            }
        }
        screen || body
    }
    fn draw(&self) {
        for i in self.blocks.iter() {
            draw_rectangle(
                (i.x * 20) as f32,
                (i.y * 20) as f32,
                20 as f32,
                20 as f32,
                if self.is_dead() { RED } else { GREEN },
            );
        }
    }
}

impl Apple {
    fn new(s: &Snake) -> Apple {
        let x = gen_range(0, SCREEN_WIDTH / BLOCK_SIZE);
        let y = gen_range(0, SCREEN_HEIGHT / BLOCK_SIZE);

        /* check if this collides with the snake's body */
        for b in s.blocks.iter() {
            if b.x == x && b.y == y {
                /* Generate anew */
                return Apple::new(s);
            }
        }
        Apple { x, y }
    }
    fn draw(&self) {
        draw_rectangle(
            (self.x * 20) as f32,
            (self.y * 20) as f32,
            20f32,
            20f32,
            RED,
        )
    }
}

fn is_out_of_bounds(x: i32, y: i32) -> bool {
    x < 0 || x >= (SCREEN_WIDTH / BLOCK_SIZE) || y < 0 || y >= (SCREEN_HEIGHT / BLOCK_SIZE)
}

fn direct_snake(s: &mut Snake) {
    s.direction = if is_key_down(KeyCode::Up) {
        Direction::Up
    } else if is_key_down(KeyCode::Left) {
        Direction::Left
    } else if is_key_down(KeyCode::Right) {
        Direction::Right
    } else if is_key_down(KeyCode::Down) {
        Direction::Down
    } else {
        s.direction.clone()
    };
}

fn draw_text_center_x(s: &str, y: f32, size: f32, c: Color) {
    let center = get_text_center(s, Option::None, size as u16, 1.0, 0.0);
    draw_text(s, SCREEN_WIDTH as f32 / 2.0 - center.x, y, size, c);
}

// Entry point.
#[macroquad::main("Snake")]
async fn main() {
    srand(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("ERROR: Can't get time")
            .as_millis() as u64,
    );
    let mut snake = Snake::new();

    request_new_screen_size(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);
    next_frame().await;

    let mut move_timer = 0;
    let mut apple = Apple::new(&snake);

    loop {
        /* Input */
        direct_snake(&mut snake);

        /* Handle movement */
        move_timer += 1;
        if move_timer % 12 == 0 {
            if !snake.is_dead() {
                snake.slither();
            }
            move_timer = 0;
        }

        /* Check apple. */
        if apple.x == snake.blocks[0].x && apple.y == snake.blocks[0].y {
            apple = Apple::new(&snake);
            snake.grow();
        }
        apple.draw();

        /* Draw stuff to the screen. */
        snake.draw();
        draw_text(
            &format!("Score: {}", snake.blocks.len()),
            20.0,
            20.0,
            24.0,
            color_u8!(255, 255, 255, 255),
        );

        if snake.is_dead() {
            draw_text_center_x("You died!", 315.0, 35.0, color_u8!(255, 0, 0, 255));
            draw_text_center_x(
                "Press spacebar to restart",
                350.0,
                35.0,
                color_u8!(255, 255, 255, 255),
            );
        }
        if is_key_down(KeyCode::Space) {
            snake = Snake::new();
        }
        next_frame().await
    }
}
