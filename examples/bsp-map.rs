use map_gen_2d::{bsp::*, Point};
use rand::SeedableRng;
fn main() {
    let map = BSPMap::new(Point::new(30,50), SeedableRng::seed_from_u64(1), Point::new(3,5), Point::new(10,15)).unwrap();
    println!("{}", map);
}