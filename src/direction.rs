use crate::{rule::*, DIRECTIONS_COUNT};

#[derive(Debug, Default)]
pub(crate) struct Direction(u8);

impl Direction {
    pub(crate) fn get(&self) -> u8 {
        self.0
    }
}

impl From<Rule> for Direction {
    fn from(value: Rule) -> Self {
        // println!("value.get(): {}", value.get());
        let out =
            Direction(((value.get() + DIRECTIONS_COUNT as i8) % (DIRECTIONS_COUNT as i8)) as u8);
        assert_eq!(out.0, out.0 % DIRECTIONS_COUNT);
        out
    }
}

impl core::ops::AddAssign for Direction {
    fn add_assign(&mut self, rhs: Self) {
        // println!("self: {:?}, rhs: {:?}", self, rhs);
        self.0 = (self.0 + rhs.0) % DIRECTIONS_COUNT;
        assert_eq!(self.0, self.0 % DIRECTIONS_COUNT);
    }
}

impl core::ops::Add for Direction {
    type Output = Direction;

    fn add(self, rhs: Self) -> Self::Output {
        let out = Direction((self.0 + rhs.0) % DIRECTIONS_COUNT);
        assert_eq!(out.0, out.0 % DIRECTIONS_COUNT);
        out
    }
}
