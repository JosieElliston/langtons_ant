mod ant;
mod direction;
mod grid;
mod rule;

use grid::*;
use rule::*;

const DIRECTIONS_COUNT: u8 = 4;
const SCREEN_SIZE: i32 = 700;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .title("ant")
        // .resizable()
        .build();

    let mut step_pow = 8;

    // let mut grid = Grid::new(vec![Rule::LEFT, Rule::RIGHT]); // langton's ant
    // let mut grid = Grid::new(vec![Rule::RIGHT, Rule::LEFT, Rule::RIGHT]); // chaotic
    let mut grid = Grid::new(vec![Rule::R, Rule::R, Rule::L, Rule::L]); // symmetric

    while !rl.window_should_close() {
        // if rl.get_time() >= 10.0 {
        //     return;
        // }

        let dt = rl.get_frame_time();
        println!("dt: {}", dt);

        // if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        //     grid.step();
        // }
        // if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        //     grid.step();
        // }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_DOWN) {
            step_pow /= 2;
        }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_UP) {
            if step_pow == 0 {
                step_pow = 1;
            } else {
                step_pow *= 2;
            }
        }
        for _ in 0..2u64.pow(step_pow) {
            grid.step();
        }

        let mut draw_handle = rl.begin_drawing(&thread);

        grid.draw(&mut draw_handle, SCREEN_SIZE);
    }
}
