use raylib::prelude::*;

use crate::{
    ant::*,
    rule::*,
    state::*,
    DIRECTIONS_COUNT,
};

// enum Grid {
//     Tri {},
//     Square {},
//     Hex {},
// }

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
    Color::DARKBLUE,
    Color::LIGHTGRAY,
    Color::GRAY,
    Color::DARKGRAY,
    Color::GOLD,
    Color::MAROON,
    Color::LIME,
    Color::DARKGREEN,
    Color::DARKPURPLE,
    Color::BEIGE,
    Color::BROWN,
    Color::DARKBROWN,
    Color::MAGENTA,
    Color::RAYWHITE,
];

#[derive(Debug)]
pub(crate) enum ResizedState {
    Resized,
    NotResized,
}

pub(crate) struct Grid {
    ant: Ant,
    rules: Vec<Rule>,
    state: State,
    // draw_updates: Vec<Coord>,
}

impl Grid {
    pub(crate) fn new(rules: Vec<Rule>, initial_radius: i32) -> Grid {
        Grid {
            ant: Ant::new(),
            rules,
            state: State::new(initial_radius),
            // draw_updates: Vec::new(),
        }
    }

    pub(crate) fn width(&self) -> usize {
        self.state.width()
    }

    pub(crate) fn step(&mut self) -> ResizedState {
        let state = self.state.get_mut_x_y(self.ant.x, self.ant.y);
        let rule = self.rules[*state as usize];
        *state += 1;
        *state %= self.rules.len() as StateDataType;
        self.ant.dir = (((self.ant.dir as i32) + rule) as u32) % DIRECTIONS_COUNT;
        match self.ant.dir {
            0 => {
                self.ant.x += 1;
                if self.ant.x > self.ant.x_max {
                    self.ant.x_max = self.ant.x;
                    if self.ant.x >= self.state.x_min() + self.state.width() as i32 {
                        self.state.resize(&self.ant);
                        return ResizedState::Resized;
                    }
                }
                ResizedState::NotResized
            }
            1 => {
                self.ant.y -= 1;
                if self.ant.y < self.ant.y_min {
                    self.ant.y_min = self.ant.y;
                    if self.ant.y < self.state.y_min() {
                        self.state.resize(&self.ant);
                        return ResizedState::Resized;
                    }
                }
                ResizedState::NotResized
            }
            2 => {
                self.ant.x -= 1;
                if self.ant.x < self.ant.x_min {
                    self.ant.x_min = self.ant.x;
                    if self.ant.x < self.state.x_min() {
                        self.state.resize(&self.ant);
                        return ResizedState::Resized;
                    }
                }
                ResizedState::NotResized
            }
            3 => {
                self.ant.y += 1;
                if self.ant.y > self.ant.y_max {
                    self.ant.y_max = self.ant.y;
                    if self.ant.y >= self.state.y_min() + self.state.width() as i32 {
                        self.state.resize(&self.ant);
                        return ResizedState::Resized;
                    }
                }
                ResizedState::NotResized
            }
            _ => panic!("invalid ant direction"),
        }
    }

    fn draw_cell(
        &self,
        draw_handle: &mut RaylibDrawHandle,
        screen_width: i32,
        rect_size: Vector2,
        state: StateDataType,
        row: usize,
        col: usize,
    ) {
        draw_handle.draw_rectangle_v(
            Vector2 {
                x: screen_width as f32 * col as f32 / self.state.width() as f32,
                y: screen_width as f32 * row as f32 / self.state.width() as f32,
            },
            rect_size,
            COLORS[state as usize % COLORS.len()],
        )
    }

    pub(crate) fn draw(&mut self, draw_handle: &mut RaylibDrawHandle, screen_width: i32) {
        let rect_size = Vector2 {
            x: screen_width as f32 / self.state.width() as f32,
            y: screen_width as f32 / self.state.width() as f32,
        };
        // if self.draw_updates.is_empty() {
        //     println!("redraw everything");
        draw_handle.clear_background(Color::BLANK);

        for row in 0..self.state.width() {
            for col in 0..self.state.width() {
                let state = *self.state.get_row_col(row, col);
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
        let mut colors: Vec<u8> = vec![0; 3 * self.state.width() * self.state.width()];

        let mut i = 0;
        for row in 0..self.state.width() {
            for col in 0..self.state.width() {
                let state = *self.state.get_row_col(row, col);
                let color = COLORS[state as usize % COLORS.len()];
                colors[i] = color.r;
                colors[i + 1] = color.g;
                colors[i + 2] = color.b;
                i += 3;
            }
        }
        image::save_buffer(
            std::path::Path::new("image.png"),
            &colors,
            self.state.width() as u32,
            self.state.width() as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}
