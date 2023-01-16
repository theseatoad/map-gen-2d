use std::fmt;

pub mod bsp;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Floor,
    Wall
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Floor => write!(f, "0"),
            Tile::Wall => write!(f, "1")
        }
    }
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Point {
    pub x : usize,
    pub y : usize,
}

impl Point {
    pub fn new(x : usize, y : usize) -> Self {
        Point {
            x,
            y
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "x: {}, y: {}\n", self.x, self.y)?;
        Ok(())
    }
}