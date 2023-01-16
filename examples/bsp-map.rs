use map_gen_2d::{bsp::*, Point};
use rand::SeedableRng;
fn main() {
    let map = BSPMap::new(Point::new(20,20), SeedableRng::seed_from_u64(1), Point::new(2,2), Point::new(8,8)).unwrap();
    println!("{}", map);
}