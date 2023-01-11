use rand::SeedableRng;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Floor,
    Wall
}

pub struct Map {
    pub size : (usize,usize),
    pub tiles : Vec<Vec<Tile>>
}

impl Map {
    pub fn new(size : (usize,usize), seed : u64) -> Self {
        let map = Map {
            size : size,
            tiles : vec![vec![Tile::Wall; size.0]; size.1],
        };
        map
    }
}