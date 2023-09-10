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

    // let mut grid = Grid::new(vec![Rule::L, Rule::R]); // langton's ant
    let mut grid = Grid::new(vec![Rule::R, Rule::L, Rule::R]); // chaotic
    // let mut grid = Grid::new(vec![Rule::R, Rule::R, Rule::L, Rule::L]); // symmetric
    // let mut grid = Grid::new(vec![
    //     Rule::L,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    // ]); // grows in a square
    // let mut grid = Grid::new(vec![
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    //     Rule::L,
    //     Rule::R,
    //     Rule::L,
    //     Rule::R,
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    // ]); // convoluted highway
    // let mut grid = Grid::new(vec![
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    //     Rule::L,
    //     Rule::R,
    //     Rule::L,
    //     Rule::R,
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    // ]); // convoluted highway
    // let mut grid = Grid::new(vec![
    //     Rule::R,
    //     Rule::R,
    //     Rule::L,
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    //     Rule::L,
    //     Rule::L,
    //     Rule::L,
    //     Rule::R,
    //     Rule::R,
    //     Rule::R,
    // ]); // triangle
    // const TEST_STEPS: u128 = 2u128.pow(26);
    // let start = std::time::Instant::now();
    // for _ in 0..TEST_STEPS {
    //     grid.step();
    // }
    // let end = std::time::Instant::now();
    // println!("time per step: {}", (end-start).as_nanos() as f64 / TEST_STEPS as f64);

    // loop {
    //     if matches!(grid.step(), StepState::Resized) {
    //         grid.save_image();
    //         println!("width: {}", grid.width());
    //         if grid.width() > 100 {
    //             break;
    //         }
    //     }
    // }

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .title("ant")
        // .resizable()
        .build();

    let mut step_pow = 1;

    while !rl.window_should_close() {
        // if rl.get_time() >= 10.0 {
        //     return;
        // }

        let dt = rl.get_frame_time();
        println!("dt: {}", dt);

        // if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        //     grid.step();
        // }
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
            grid.save_image();
        }
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
            if matches!(grid.step(), StepState::Resized) {
                break;
            }
        }

        let mut draw_handle = rl.begin_drawing(&thread);

        grid.draw(&mut draw_handle, SCREEN_SIZE);
    } 
}
