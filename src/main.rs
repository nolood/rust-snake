use macroquad::prelude::*;
use rand::Rng;
extern crate rand;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 15;
const MOVE_DELAY: f32 = 0.2;

#[macroquad::main("Snake")]
async fn main() {
    let mut snake = vec![(20.0, 20.0)];

    let mut last_move_time = get_time();

    let mut rng = rand::rng();

    let mut food = (20.0, 20.0);

    food.0 = (rng.random_range(10..GRID_WIDTH) * CELL_SIZE as i32) as f32;
    food.1 = (rng.random_range(10..GRID_HEIGHT) * CELL_SIZE as i32) as f32;

    let mut direction = (CELL_SIZE, 0.0);

    println!("{:?}", food);

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::Right) {
            direction = (CELL_SIZE, 0.0);
        }
        if is_key_down(KeyCode::Left) {
            direction = (-CELL_SIZE, 0.0);
        }
        if is_key_down(KeyCode::Up) {
            direction = (0.0, -CELL_SIZE);
        }
        if is_key_down(KeyCode::Down) {
            direction = (0.0, CELL_SIZE);
        }

        if get_time() - last_move_time > MOVE_DELAY as f64 {
            let head = snake[0];
            let new_head = (head.0 + direction.0, head.1 + direction.1);

            snake.insert(0, new_head);

            if new_head.0 == food.0 && new_head.1 == food.1 {
                food.0 = (rng.random_range(0..GRID_WIDTH) * CELL_SIZE as i32) as f32;
                food.1 = (rng.random_range(0..GRID_HEIGHT) * CELL_SIZE as i32) as f32;
            } else {
                snake.pop();
            }

            last_move_time = get_time();
        }

        for (x, y) in &snake {
            draw_rectangle(*x, *y, CELL_SIZE, CELL_SIZE, GREEN);
        }

        println!("{:?}", snake);

        draw_rectangle(food.0, food.1, CELL_SIZE, CELL_SIZE, RED);

        next_frame().await;
    }
}
