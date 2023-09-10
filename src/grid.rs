use raylib::prelude::*;

use crate::{ant::*, rule::*};

// enum Grid {
//     Tri {

//     },
//     Square {

//     },
//     Hex {

//     },
// }

const INITIAL_RADIUS: i32 = 4;
// const RESIZE_BUFFER: i32 = 4;
const RESIZE_BUFFER: f32 = 1.1;

static COLORS: &[Color] = &[
    Color::BLACK,
    Color::WHITE,
    Color::VIOLET,
    Color::BLUE,
    Color::GREEN,
    Color::YELLOW,
    Color::RED,
    Color::ORANGE,
    Color::PURPLE,
    Color::PINK,
    Color::SKYBLUE,
    Color::LIME,
    Color::GOLD,
    Color::BROWN,
    Color::MAROON,
];

// struct Coord {
//     row: usize,
//     col: usize,
// }

pub(crate) enum StepState {
    Resized,
    NotResized,
}

pub(crate) struct Grid {
    ant: Ant,
    rule_len: u8,
    rules: Vec<Rule>,
    // colors: Vec<Color>,
    state_x_min: i32,
    state_y_min: i32,
    state_width: i32,
    state: Vec<Vec<u8>>,
    // draw_updates: Vec<Coord>,
}

impl Grid {
    pub(crate) fn new(rules: Vec<Rule>) -> Grid {
        Grid {
            ant: Ant::new(),
            rule_len: rules.len() as u8,
            rules,
            state_x_min: -INITIAL_RADIUS,
            state_y_min: -INITIAL_RADIUS,
            state_width: 2 * INITIAL_RADIUS,
            state: vec![vec![0; 2 * INITIAL_RADIUS as usize]; 2 * INITIAL_RADIUS as usize],
            // draw_updates: Vec::new(),
        }
    }

    pub(crate) fn width(&self) -> i32 {
        self.state_width
    }

    fn resize(&mut self) {
        println!("resize");
        let center_x = (self.ant.x_max() + self.ant.x_min()) / 2;
        let center_y = (self.ant.y_max() + self.ant.y_min()) / 2;
        let new_radius = (((center_x - self.ant.x_min()).abs())
            .max((center_x - self.ant.x_max()).abs())
            .max((center_y - self.ant.y_min()).abs())
            .max((center_y - self.ant.y_max()).abs()) as f32
            * RESIZE_BUFFER) as i32;

        let new_width = 2 * new_radius + 1;
        let new_state_x_min = center_x - new_radius;
        let new_state_y_min = center_y - new_radius;
        let new_state_x_max = center_x + new_radius;
        let new_state_y_max = center_y + new_radius;
        assert_eq!(new_state_x_max, new_state_x_min + new_width - 1);
        assert_eq!(new_state_y_max, new_state_y_min + new_width - 1);

        assert!(new_state_x_min <= self.ant.x_min());
        assert!(new_state_y_min <= self.ant.y_min());
        assert!(new_state_x_min + new_width > self.ant.x_max());
        assert!(new_state_y_min + new_width > self.ant.y_max());
        assert!(
            !((self.ant.x() < new_state_x_min)
                || (self.ant.y() < new_state_y_min)
                || (self.ant.x() >= new_state_x_min + new_width)
                || (self.ant.y() >= new_state_y_min + new_width))
        );

        let mut new_state: Vec<Vec<u8>> = vec![vec![0; new_width as usize]; new_width as usize];
        for (row, line) in self.state.iter().enumerate() {
            for (col, &val) in line.iter().enumerate() {
                let row1 = row as i32 - new_state_y_min + self.state_y_min;
                if row1 < 0 || row1 >= new_width {
                    continue;
                }
                let col1 = col as i32 - new_state_x_min + self.state_x_min;
                if col1 < 0 || col1 >= new_width {
                    continue;
                }
                new_state[row1 as usize][col1 as usize] = val;
            }
        }

        self.state_x_min = new_state_x_min;
        self.state_y_min = new_state_y_min;
        // assert!(self.state_x_min <= self.ant.x_min());
        // assert!(self.state_y_min <= self.ant.y_min());
        // assert!(self.state_x_min + self.state_width > self.ant.x_max());
        // assert!(self.state_y_min + self.state_width > self.ant.y_max());
        // assert!(!((self.ant.x() < self.state_x_min)
        // || (self.ant.y() < self.state_y_min)
        // || (self.ant.x() >= self.state_x_min + self.state_width)
        // || (self.ant.y() >= self.state_y_min + self.state_width)));
        self.state_width = new_width;
        self.state = new_state;
    }

    pub(crate) fn step(&mut self) -> StepState {
        let row = (self.ant.y() - self.state_y_min) as usize;
        let col = (self.ant.x() - self.state_x_min) as usize;
        // let row = (self.ant.y() + self.neg_coord_min as i32) as usize;
        // let col = (self.ant.x() + self.neg_coord_min as i32) as usize;
        let state = &mut self.state[row][col];
        let rule = self.rules[*state as usize];
        *state += 1;
        *state %= self.rule_len;
        self.ant.step(rule);
        if (self.ant.x() < self.state_x_min)
            || (self.ant.y() < self.state_y_min)
            || (self.ant.x() >= self.state_x_min + self.state_width)
            || (self.ant.y() >= self.state_y_min + self.state_width)
        {
            self.resize();
            StepState::Resized
        } else {
            StepState::NotResized
        }
    }

    fn draw_cell(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        screen_width: i32,
        rect_size: Vector2,
        state: u8,
        row: usize,
        col: usize,
    ) {
        draw_handle.draw_rectangle_v(
            Vector2 {
                x: screen_width as f32 * col as f32 / self.state_width as f32,
                y: screen_width as f32 * row as f32 / self.state_width as f32,
            },
            rect_size,
            COLORS[state as usize],
        )
    }

    pub(crate) fn draw(&mut self, draw_handle: &mut RaylibDrawHandle, screen_width: i32) {
        let rect_size = Vector2 {
            x: screen_width as f32 / self.state_width as f32,
            y: screen_width as f32 / self.state_width as f32,
        };
        // if self.draw_updates.is_empty() {
        //     println!("redraw everything");
        draw_handle.clear_background(Color::BLANK);

        for (row, line) in self.state.iter().enumerate() {
            for (col, &state) in line.iter().enumerate() {
                if state == 0 {
                    continue;
                }
                self.draw_cell(draw_handle, screen_width, rect_size, state, row, col);
            }
        }
        // } else {
        //     for coord in &self.draw_updates {
        //         self.draw_cell(
        //             draw_handle,
        //             screen_width,
        //             rect_size,
        //             self.state[coord.row][coord.col],
        //             coord.row,
        //             coord.col,
        //         );
        //     }
        // }
        // self.draw_updates.clear();
    }

    pub(crate) fn save_image(&self) {
        let mut colors: Vec<u8> =
            vec![0; (3 * self.state_width * self.state_width) as usize];

        let mut i = 0;
        for line in &self.state {
            for &state in line {
                let color = COLORS[state as usize];
                colors[i] = color.r;
                colors[i+1] = color.g;
                colors[i+2] = color.b;
                i += 3;
            }
        }
        assert_eq!(self.state.len(), self.state_width as usize);
        image::save_buffer(
            std::path::Path::new("image.png"),
            &colors,
            self.state_width as u32,
            self.state_width as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(vec![Rule::L, Rule::R])
    }
}
