use map_gen_2d::{bsp::*, Map, Point};
use rand::SeedableRng;
fn main() {
    let map = BSPMap::new(Point::new(25,25), SeedableRng::seed_from_u64(124)).unwrap();
    println!("{}", map);
}