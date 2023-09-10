use crate::{direction::*, rule::*};

pub(crate) struct Ant {
    x: i32,
    y: i32,
    dir: Direction,
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

impl Ant {
    pub(crate) fn new() -> Ant {
        Ant {
            x: 0,
            y: 0,
            dir: Direction::default(),
            x_min: 0,
            y_min: 0,
            x_max: 0,
            y_max: 0,
        }
    }

    pub(crate) fn x(&self) -> i32 {
        self.x
    }
    pub(crate) fn y(&self) -> i32 {
        self.y
    }
    pub(crate) fn x_min(&self) -> i32 {
        self.x_min
    }
    pub(crate) fn y_min(&self) -> i32 {
        self.y_min
    }
    pub(crate) fn x_max(&self) -> i32 {
        self.x_max
    }
    pub(crate) fn y_max(&self) -> i32 {
        self.y_max
    }

    // pub(crate) fn explored_radius(&self) -> i32 {
    //     ((self.x - self.x_min).abs())
    //         .max((self.x - self.x_max).abs())
    //         .max((self.y - self.y_min).abs())
    //         .max((self.y - self.y_max).abs())
    // }

    pub(crate) fn step(&mut self, rule: Rule) {
        self.dir += rule.into();
        match self.dir.get() {
            0 => {
                self.x += 1;
                self.x_max = self.x.max(self.x_max);
                // if self.x > self.x_max {
                //     self.x_max = self.x;
                // }
            }
            1 => {
                self.y -= 1;
                self.y_min = self.y.min(self.y_min);
                // if self.y < self.y_min {
                //     self.y_min = self.y;
                // }
            }
            2 => {
                self.x -= 1;
                self.x_min = self.x.min(self.x_min);
                // if self.x < self.x_min {
                //     self.x_min = self.x;
                // }
            }
            3 => {
                self.y += 1;
                self.y_max = self.y.max(self.y_max);
                // if self.y > self.y_max {
                //     self.y_max = self.y;
                // }
            }
            _ => panic!("invalid ant direction"),
        }
    }
}
