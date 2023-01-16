use map_gen_2d::{bsp::*, Point};
use rand::SeedableRng;
fn main() {
    let map = BSPMap::new(Point::new(20,50), SeedableRng::seed_from_u64(1), Point::new(2,5), Point::new(10,15)).unwrap();
    println!("{}", map);
}