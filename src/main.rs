mod ant;
mod grid;
mod rule;
mod state;

use std::hint::black_box;

use grid::*;
use rule::*;

const DIRECTIONS_COUNT: u32 = 4;
const SCREEN_SIZE: i32 = 700;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut grid = Grid::new(
        // rule_vec_from_str("LR"), // langton's ant
        // rule_vec_from_str("RLR"), // chaotic
        rule_vec_from_str("RRLL"), // symmetric cardioid
        // rule_vec_from_str("RLLR"), // symmetric square
        // rule_vec_from_str("LRRRRRLLR"), // grows in a square
        // rule_vec_from_str("LLRRRLRLRLLR"), // convoluted highway
        // rule_vec_from_str("RRLLLRLLLRRR"), // triangle
        // rule_vec_from_str("LLRLRRRLRRLLLRRLLLLLLRLLRLRR"), // long period
        // rule_vec_from_str("LLRLRRLLRLLLLLLRRLLLR"), // long period 2
        // rule_vec_from_str("RRLRLLRRLRRRRRRRRRLLLRLLRLR"), // long period 3
        // rule_vec_from_str("RRRLRRLLRLRRRRLLRRRLLRLR"), // long period 4
        // rule_vec_from_str("RRRLRRLLRLRRRRRRRRRLR"), // long period 5
        // rule_vec_from_str("LLRLRRLLRLLLLLLLLRRRRRRLRLR"), // long period 6
        // rule_vec_from_str("RRLRLLRRLRRRRRRRRRLRRLRLRR"), // long period 7
        // rule_vec_from_str("LLRLRRLLRLLLLLLLLRRRLLRLR"), // long period 8
        // rule_vec_from_str("LLRLRRLLRLLLLLLLLLLRLRLR"), // long period 9
        // rule_vec_from_str("RRLRLLRRLRRRRRRRRRLLLLRLRR"), // long period 10
        4,
    );

    // simple test
    // const TEST_STEPS: u128 = 2u128.pow(26);
    // for _ in 0..1000 {
    //     grid.step();
    // }
    // let start = std::time::Instant::now();
    // for _ in 0..TEST_STEPS {
    //     black_box(grid.step());
    //     // grid.step();
    // }
    // let end = std::time::Instant::now();
    // println!(
    //     "time per step: {}",
    //     (end - start).as_nanos() as f64 / TEST_STEPS as f64
    // );

    // average of samples
    // black_box makes it faster???
    // const TEST_N: u128 = 1000;
    // const TEST_STEPS: u128 = 100000;
    // let mut hist = histo::Histogram::with_buckets(200);
    // for _ in 0..TEST_N {
    //     let mut grid = Grid::new(rule_vec_from_str("RRLL"), 200);
    //     for _ in 0..100 {
    //         grid.step();
    //     }
    //     let start = std::time::Instant::now();
    //     for _ in 0..TEST_STEPS {
    //         black_box(grid.step());
    //         // grid.step();
    //     }
    //     let end = std::time::Instant::now();
    //     let test_time = (end - start).as_nanos();
    //     if test_time / TEST_STEPS > 20 {
    //         continue;
    //     }
    //     hist.add(test_time as u64);
    //     // println!(
    //     //     "time for test: {}",
    //     //     (end - start).as_nanos()
    //     // );
    //     // println!(
    //     //     "time per step: {}",
    //     //     (end - start).as_nanos() as f64 / TEST_STEPS as f64
    //     // );
    // }
    // println!("{}", hist);

    // rendering: saves the image each time it gets resized
    // loop {
    //     if matches!(grid.step(), ResizedState::Resized) {
    //         grid.save_image();
    //         println!("saved image with width: {}", grid.width());
    //     }
    // }

    // interactive
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_SIZE, SCREEN_SIZE)
        .title("ant")
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
            if matches!(grid.step(), ResizedState::Resized) {
                break;
            }
        }

        let mut draw_handle = rl.begin_drawing(&thread);
        grid.draw(&mut draw_handle, SCREEN_SIZE);
    }
}
