pub(crate) struct Ant {
    pub(crate) dir: u32,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) x_min: i32,
    pub(crate) y_min: i32,
    pub(crate) x_max: i32,
    pub(crate) y_max: i32,
}

impl Ant {
    pub(crate) fn new() -> Ant {
        Ant {
            x: 0,
            y: 0,
            dir: 0,
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
}
