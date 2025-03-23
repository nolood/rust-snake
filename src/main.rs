use macroquad::prelude::*;
use rand::Rng;
extern crate rand;
// use rand::Rng;

const CELL_SIZE: f32 = 20.0;
const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const MOVE_DELAY: f32 = 0.5;
#[macroquad::main("Snake")]
async fn main() {
    let mut snake = vec![(10.0, 10.0)];

    let mut last_move_time = get_time();

    let mut rng = rand::rng();

    let mut food_x = (rng.random_range(0..GRID_WIDTH) as f32) * CELL_SIZE;
    let mut food_y = (rng.random_range(0..GRID_HEIGHT) as f32) * CELL_SIZE;

    let mut direction = (CELL_SIZE, 0.0);

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

            // Добавляем новый сегмент в начало змейки
            snake.insert(0, new_head);

            // Проверка на столкновение с едой
            if new_head.0 == food_x && new_head.1 == food_y {
                // Генерация новой еды
                food_x = (rng.random_range(0..GRID_WIDTH) as f32) * CELL_SIZE;
                food_y = (rng.random_range(0..GRID_HEIGHT) as f32) * CELL_SIZE;
            } else {
                // Удаляем последний сегмент змейки, если не съедена еда
                snake.pop();
            }

            last_move_time = get_time();
        }

        for (x, y) in &snake {
            draw_rectangle(*x, *y, CELL_SIZE, CELL_SIZE, GREEN);
        }

        // Отображаем еду
        draw_rectangle(food_x, food_y, CELL_SIZE, CELL_SIZE, RED);

        next_frame().await
    }
}
