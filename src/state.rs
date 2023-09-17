use crate::ant::*;

const RESIZE_BUFFER: f32 = 1.1;

pub(crate) type StateDataType = u8;

pub(crate) struct State {
    x_min: i32,
    y_min: i32,
    width: usize,
    data: Vec<Vec<StateDataType>>,
    // data: Vec<u8>,
}

impl State {
    pub(crate) fn new(initial_radius: i32) -> State {
        let width = 2 * initial_radius as usize;
        State {
            x_min: -initial_radius,
            y_min: -initial_radius,
            width,
            data: vec![vec![0; 2 * initial_radius as usize]; 2*initial_radius as usize],
            // data: vec![0; 4 * initial_radius as usize * initial_radius as usize],
            // data: block_grid::BlockGrid::new(width, width).unwrap(),
        }
    }

    pub(crate) fn get_row_col(&self, row: usize, col: usize) -> &StateDataType {
        assert!((0..self.width).contains(&row));
        assert!((0..self.width).contains(&col));
        &self.data[row][col]
        // &self.data[row*self.width + col]
        // self.data.get((row, col)).unwrap()
        // &self.data[zorder::index_of((row as u16, col as u16)) as usize]
    }

    pub(crate) fn get_x_y(&self, x: i32, y: i32) -> &StateDataType {
        let row = (y - self.y_min) as usize;
        let col = (x - self.x_min) as usize;
        self.get_row_col(row, col)
    }

    pub(crate) fn get_mut_row_col(&mut self, row: usize, col: usize) -> &mut StateDataType {
        assert!((0..self.width).contains(&row));
        assert!((0..self.width).contains(&col));
        &mut self.data[row][col]
        // &mut self.data[row*self.width + col]
        // self.data.get_mut((row, col)).unwrap()
        // &mut self.data[zorder::index_of((row as u16, col as u16)) as usize]
    }

    pub(crate) fn get_mut_x_y(&mut self, x: i32, y: i32) -> &mut StateDataType {
        let row = (y - self.y_min) as usize;
        let col = (x - self.x_min) as usize;
        self.get_mut_row_col(row, col)
    }

    pub(crate) fn resize(&mut self, ant: &Ant) {
        // panic!("not implemented");
        println!("resize");
        let center_x = (ant.x_max() + ant.x_min()) / 2;
        let center_y = (ant.y_max() + ant.y_min()) / 2;
        let new_radius = (((center_x - ant.x_min()).abs())
            .max((center_x - ant.x_max()).abs())
            .max((center_y - ant.y_min()).abs())
            .max((center_y - ant.y_max()).abs()) as f32
            * RESIZE_BUFFER) as i32;

        let new_width = 2 * new_radius + 1;
        let new_x_min = center_x - new_radius;
        let new_y_min = center_y - new_radius;
        let new_state_x_max = center_x + new_radius;
        let new_state_y_max = center_y + new_radius;
        assert_eq!(new_state_x_max, new_x_min + new_width - 1);
        assert_eq!(new_state_y_max, new_y_min + new_width - 1);

        assert!(new_x_min <= ant.x_min());
        assert!(new_y_min <= ant.y_min());
        assert!(new_x_min + new_width > ant.x_max());
        assert!(new_y_min + new_width > ant.y_max());
        assert!(
            !((ant.x() < new_x_min)
                || (ant.y() < new_y_min)
                || (ant.x() >= new_x_min + new_width)
                || (ant.y() >= new_y_min + new_width))
        );

        let mut new_state: Vec<Vec<StateDataType>> = vec![vec![0; new_width as usize]; new_width as usize];
        for row in 0..self.width {
            for col in 0..self.width {
                let row1 = row as i32 - new_y_min + self.y_min;
                if row1 < 0 || row1 >= new_width {
                    continue;
                }
                let col1 = col as i32 - new_x_min + self.x_min;
                if col1 < 0 || col1 >= new_width {
                    continue;
                }
                let val = *self.get_row_col(row, col);
                new_state[row1 as usize][col1 as usize] = val;
            }
        }

        self.x_min = new_x_min;
        self.y_min = new_y_min;
        self.width = new_width as usize;
        self.data = new_state;

        assert!(ant.x() >= self.x_min);
        assert!(ant.y() >= self.y_min);
        assert!(ant.x() < self.x_min + self.width as i32);
        assert!(ant.y() < self.y_min + self.width as i32);
    }

    pub(crate) fn width(&self) -> usize {
        self.width
    }
    pub(crate) fn x_min(&self) -> i32 {
        self.x_min
    }
    pub(crate) fn y_min(&self) -> i32 {
        self.y_min
    }
    // kinda ambiguous if this is should be inclusive or exclusive so im just not providing these
    // pub(crate) fn x_max(&self) -> i32 {
    //     self.x_min + self.width as i32
    // }
    // pub(crate) fn y_max(&self) -> i32 {
    //     self.y_min + self.width as i32
    // }
}
