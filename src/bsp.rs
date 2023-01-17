use core::fmt;
use std::collections::HashMap;

use crate::{Point, Tile};
use anyhow::bail;
use rand::{rngs::StdRng, Rng};
/// Map generated with binary search partitioning. The map must have a size of atleast (20,20).
///
/// Credit to https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268 and https://github.com/whostolemyhat/dungeon
/// for algorithm and rust implementation help.
pub struct BSPMap {
    size: Point,
    tiles: HashMap<Point, Tile>,
    rooms: Vec<Room>,
    min_room_size: Point,
    max_room_size: Point,
}
impl BSPMap {
    pub fn new(
        size: Point,
        mut seed: StdRng,
        min_room_size: Point,
        max_room_size: Point,
    ) -> Result<Self, anyhow::Error> {
        if size.x < 20 || size.y < 20 {
            bail!("Size of a BSP_Map needs to be greater than or equal x : 20, y : 20")
        }
        if min_room_size.x >= max_room_size.x {
            bail!("Minimum room size (x) needs to be less than maximum room size (x).")
        }
        if min_room_size.y >= max_room_size.y {
            bail!("Minimum room size (y) needs to be less than maximum room size (y).")
        }
        if max_room_size.x >= size.x {
            bail!("Maximum room size (x) must be less than map size (x).")
        }
        if max_room_size.y >= size.y {
            bail!("Maximum room size (y) must be less than map size (y).")
        }
        let mut map = BSPMap {
            size: size,
            tiles: HashMap::new(),
            rooms: Vec::new(),
            min_room_size,
            max_room_size,
        };
        map.place_rooms(&mut seed, map.min_room_size, map.max_room_size);
        // Hard wall around left
        for y in 0..size.y {
            map.tiles.insert(Point::new(0, y), Tile::Wall);
            map.tiles.insert(Point::new(map.size.x, y), Tile::Wall);

        }
        // hard wall on top
        for x in 0..size.x {
            map.tiles.insert(Point::new(x, 0), Tile::Wall);
            map.tiles.insert(Point::new(x, map.size.y), Tile::Wall);
        }
        let mut walls : Vec<Point> = Vec::new();
        // Walls around tiles
        for tile in map.tiles.iter() {
            // check all eight points around
            if map.tiles.get(&Point::new(tile.0.x + 1, tile.0.y)).is_none() {
               walls.push(Point::new(tile.0.x + 1, tile.0.y))
            }
            if map.tiles.get(&Point::new(tile.0.x + 1, tile.0.y + 1)).is_none() {
                walls.push(Point::new(tile.0.x + 1, tile.0.y + 1))
            }
            if map.tiles.get(&Point::new(tile.0.x, tile.0.y + 1)).is_none() {
                walls.push(Point::new(tile.0.x, tile.0.y + 1))
            }
            if tile.0.x != 0 && map.tiles.get(&Point::new(tile.0.x - 1, tile.0.y + 1)).is_none() {
                walls.push(Point::new(tile.0.x - 1, tile.0.y + 1))
            }
            if tile.0.x != 0 && map.tiles.get(&Point::new(tile.0.x - 1, tile.0.y)).is_none() {
                walls.push(Point::new(tile.0.x - 1, tile.0.y))
            }
            if tile.0.x != 0 && tile.0.y != 0 && map.tiles.get(&Point::new(tile.0.x - 1, tile.0.y - 1)).is_none() {
                walls.push(Point::new(tile.0.x - 1, tile.0.y - 1))
            }
            if tile.0.y != 0 && map.tiles.get(&Point::new(tile.0.x, tile.0.y - 1)).is_none() {
                walls.push(Point::new(tile.0.x, tile.0.y - 1))
            }
            if tile.0.y != 0 && map.tiles.get(&Point::new(tile.0.x + 1, tile.0.y - 1)).is_none() {
                walls.push(Point::new(tile.0.x + 1, tile.0.y - 1))
            }
        }
        // Insert walls
        for wall in walls.iter(){
            map.tiles.insert(wall.clone(), Tile::Wall);
        }
        Ok(map)
    }

    pub fn get_tiles(&self) -> &HashMap<Point, Tile> {
        &self.tiles
    }

    pub fn get_size(&self) -> &Point {
        &self.size
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &&self.rooms
    }
    fn place_rooms(&mut self, rng: &mut StdRng, min_room_size: Point, max_room_size: Point) {
        let mut root = Leaf::new(Point { x: 0, y: 0 }, self.size);
        // generate leaves
        root.generate(rng, &min_room_size, &max_room_size);
        // generate rooms in leaves
        root.create_rooms(rng, &min_room_size);
        // Loop over leaves spawning rooms
        for leaf in root.iter() {
            if leaf.is_leaf() {
                if let Some(room) = leaf.get_room() {
                    self.add_room(&room);
                }
            }

            for corridor in &leaf.corridors {
                self.add_room(&corridor);
            }
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for x in 0..room.size.x {
            for y in 0..room.size.y {
                self.tiles.insert(
                    Point::new(room.position.x + x, room.position.y + y),
                    Tile::Floor,
                );
            }
        }
        self.rooms.push(room.clone());
    }
}

impl fmt::Display for BSPMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // for tile in self.tiles.iter() {
        //     match self.tiles.get(&tile.0) {
        //         Some(x) => write!(f, "{}", x)?,
        //         None => write!(f, "x")?,
        //     }
        // }
        for row in 0..=self.size.x {
            for col in 0..=self.size.y {
                match self.tiles.get(&Point::new(row, col)) {
                    Some(x) => write!(f, "{}", x)?,
                    None => write!(f, "x")?,
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Room {
    position: Point,
    size: Point,
}

impl Room {
    pub fn new(position: Point, size: Point) -> Self {
        Room { position, size }
    }
    /// Calculates if two rooms intersect with one another.
    /// Rust port of https://stackoverflow.com/questions/20925818/algorithm-to-check-if-two-boxes-overlap
    pub fn intersects(&self, other: &Room) -> bool {
        let x_intersect: bool = ((self.position.x + self.size.x) > other.position.x)
            && (other.position.x + (other.size.x) > self.position.x);
        let y_intersect: bool = ((self.position.y + self.size.y) > other.position.y)
            && (other.position.y + (other.size.y) > self.position.y);
        x_intersect && y_intersect
    }
}

pub struct Leaf {
    /// Top left corner (x,y)
    position: Point,
    /// Size of leaf (x,y)
    size: Point,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>,
    corridors: Vec<Room>,
}

impl Leaf {
    pub fn new(position: Point, size: Point) -> Self {
        Leaf {
            position,
            size,
            left_child: None,
            right_child: None,
            room: None,
            corridors: Vec::new(),
        }
    }
    pub fn split(
        &mut self,
        rng: &mut StdRng,
        min_room_size: &Point,
        max_room_size: &Point,
    ) -> bool {
        if self.left_child.is_some() || self.right_child.is_some() {
            return false;
        }
        // init
        let split_horizontal: bool;
        // determine split direction
        if (self.size.x > self.size.y) && (self.size.x as f32 / self.size.y as f32) >= 1.25 {
            split_horizontal = false;
        } else if (self.size.y > self.size.x) && (self.size.y as f32 / self.size.x as f32) >= 1.25 {
            split_horizontal = true;
        } else {
            split_horizontal = rng.gen_bool(0.5);
        };

        // determine where we are going to split
        let split = if split_horizontal == true {
            rng.gen_range(min_room_size.x..=max_room_size.y as usize)
        } else {
            rng.gen_range(min_room_size.y..=max_room_size.y as usize)
        };
        // split
        if split_horizontal {
            self.left_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x, self.position.y),
                Point::new(self.size.x, split),
            )));
            if split >= self.size.y {
                return false;
            } else {
                self.right_child = Some(Box::new(Leaf::new(
                    Point::new(self.position.x, self.position.y + split),
                    Point::new(self.size.x, self.size.y - split),
                )));
            }
        } else {
            self.left_child = Some(Box::new(Leaf::new(
                Point::new(self.position.x, self.position.y),
                Point::new(split, self.size.y),
            )));
            if split >= self.size.x {
                return false;
            } else {
                self.right_child = Some(Box::new(Leaf::new(
                    Point::new(self.position.x + split, self.position.y),
                    Point::new(self.size.x - split, self.size.y),
                )));
            }
        }
        true
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => match self.right_child {
                None => true,
                Some(_) => false,
            },
            Some(_) => false,
        }
    }

    fn generate(&mut self, rng: &mut StdRng, min_room_size: &Point, max_room_size: &Point) {
        if self.is_leaf() {
            if self.split(rng, min_room_size, max_room_size) {
                self.left_child
                    .as_mut()
                    .unwrap()
                    .generate(rng, min_room_size, max_room_size);
                self.right_child
                    .as_mut()
                    .unwrap()
                    .generate(rng, min_room_size, max_room_size);
            }
        }
    }

    fn create_rooms(&mut self, rng: &mut StdRng, min_room_size: &Point) {
        // If it is not a end leaf.
        if let Some(ref mut room) = self.left_child {
            room.as_mut().create_rooms(rng, min_room_size);
        };
        // If it is not a end leaf.
        if let Some(ref mut room) = self.right_child {
            room.as_mut().create_rooms(rng, min_room_size);
        };

        // if last level, add a room
        if self.is_leaf() {
            let width: usize;
            if min_room_size.x >= self.size.x {
                width = min_room_size.x;
            } else {
                width = rng.gen_range(min_room_size.x..=self.size.x);
            }

            let height: usize;
            if min_room_size.y >= self.size.y {
                height = min_room_size.y;
            } else {
                height = rng.gen_range(min_room_size.y..=self.size.y);
            }
            let x: usize;
            if self.size.x as f32 - width as f32 <= 0.0 {
                x = 0
            } else {
                x = rng.gen_range(0..=(self.size.x - width));
            }

            let y: usize;
            if self.size.y as f32 - height as f32 <= 0.0 {
                y = 0
            } else {
                y = rng.gen_range(0..=(self.size.y - height));
            }
            self.room = Some(Room::new(
                Point::new(x + self.position.x, y + self.position.y),
                Point::new(width, height),
            ));
        }
        if let (Some(ref mut left), Some(ref mut right)) =
            (&mut self.left_child, &mut self.right_child)
        {
            create_corridors(rng, left, right);
        };
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room;
        }

        let mut left_room: Option<Room> = None;
        let mut right_room: Option<Room> = None;

        if let Some(ref room) = self.left_child {
            left_room = room.get_room();
        }

        if let Some(ref room) = self.right_child {
            right_room = room.get_room();
        }
        match (left_room, right_room) {
            (None, None) => None,
            (Some(room), _) => Some(room),
            (_, Some(room)) => Some(room),
        }
    }

    fn iter(&self) -> LeafIterator {
        LeafIterator::new(&self)
    }
}

fn create_corridors(rng: &mut StdRng, left: &mut Box<Leaf>, right: &mut Box<Leaf>) {
    if let (Some(left_room), Some(right_room)) = (left.get_room(), right.get_room()) {
        // Get random x position and y position
        let left_point = Point::new(
            rng.gen_range(left_room.position.x..=(left_room.position.x + left_room.size.x)),
            rng.gen_range(left_room.position.y..=(left_room.position.y + left_room.size.y)),
        );
        // Get random x position and y position
        let right_point = Point::new(
            rng.gen_range(right_room.position.x..=(right_room.position.x + right_room.size.x)),
            rng.gen_range(right_room.position.y..=(right_room.position.y + right_room.size.y)),
        );

        if left_point.y <= right_point.y {
            left.corridors.push(vert_corridor(left_point.x, left_point.y, right_point.y));
        } else {
            left.corridors.push(vert_corridor(left_point.x, right_point.y, left_point.y));
        }

        if left_point.x <= right_point.x {
            left.corridors.push(horz_corridor(left_point.x, right_point.y, right_point.x));
        } else {
            left.corridors.push(horz_corridor(right_point.x, right_point.y, left_point.x));
        }
    };
}

fn horz_corridor(start_x: usize, start_y: usize, end_x: usize) -> Room {
    Room::new(Point { x: start_x, y: start_y }, Point { x : end_x - start_x + 1, y : 1})
}

fn vert_corridor(start_x: usize, start_y: usize, end_y: usize) -> Room {
    Room::new(Point { x: start_x, y: start_y }, Point { x: 1, y : end_y - start_y})
}

struct LeafIterator<'a> {
    current_node: Option<&'a Leaf>,
    right_nodes: Vec<&'a Leaf>,
}

impl<'a> LeafIterator<'a> {
    fn new(root: &'a Leaf) -> LeafIterator<'a> {
        let mut iter = LeafIterator {
            right_nodes: vec![],
            current_node: None,
        };

        iter.add_left_subtree(root);
        iter
    }

    fn add_left_subtree(&mut self, node: &'a Leaf) {
        if let Some(ref left) = node.left_child {
            self.right_nodes.push(&*left);
        }
        if let Some(ref right) = node.right_child {
            self.right_nodes.push(&*right);
        }

        self.current_node = Some(node);
    }
}

impl<'a> Iterator for LeafIterator<'a> {
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.take();
        if let Some(rest) = self.right_nodes.pop() {
            self.add_left_subtree(rest);
        }

        match result {
            Some(leaf) => Some(&*leaf),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use rand::SeedableRng;

    use crate::Point;

    use super::{Leaf, Room};

    // === Room intersect testing ===
    #[test]
    fn non_intersect_touching_walls() {
        let room_one = Room::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        let room_two = Room::new(Point { x: 10, y: 10 }, Point { x: 10, y: 10 });
        assert_eq!(false, room_one.intersects(&room_two));
    }

    #[test]
    fn non_intersect_nothing_touch() {
        let room_one = Room::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        let room_two = Room::new(Point { x: 15, y: 15 }, Point { x: 10, y: 10 });
        assert_eq!(false, room_one.intersects(&room_two));
    }

    #[test]
    fn intersect_top_right_corner() {
        let room_one = Room::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        let room_two = Room::new(Point { x: 9, y: 9 }, Point { x: 123, y: 321 });
        assert_eq!(true, room_one.intersects(&room_two));
    }

    #[test]
    fn full_intersect() {
        let room_one = Room::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        let room_two = Room::new(Point { x: 0, y: 0 }, Point { x: 20, y: 20 });
        assert_eq!(true, room_one.intersects(&room_two));
    }

    #[test]
    fn intersect_up_shift() {
        let room_one = Room::new(Point { x: 0, y: 0 }, Point { x: 10, y: 10 });
        let room_two = Room::new(Point { x: 0, y: 5 }, Point { x: 10, y: 10 });
        assert_eq!(true, room_one.intersects(&room_two));
    }
    // === Leaf split testing ===
    // Split conditions:
    // - Will not split if leaf size (x or y) is less than 1/15  (x or y) of the map
    // - Split horizontal if leaf height is >125% of width.
    // - Split vertical if leaf width is >125%  of height.
    // - Random split if both leaf width and height are within 125% of each other.
    #[test]
    fn test_no_split_too_small_horz() {}

    #[test]
    fn test_no_split_too_small_vert() {}

    #[test]
    fn test_leaf_split_horz() {
        // Vertically large room
        let mut vert_room = Leaf::new(Point::new(0, 0), Point::new(20, 50));

        let split = vert_room.split(
            &mut SeedableRng::seed_from_u64(123),
            &Point::new(10, 10),
            &Point::new(15, 15),
        );
        assert_eq!(split, true);
        let left_child = vert_room.left_child.unwrap();
        let right_child = vert_room.right_child.unwrap();

        // Now that we have split horizontally, the left child should be moved to the
        assert_eq!(left_child.position, vert_room.position);
        assert_eq!(
            left_child.size.x + left_child.size.y <= vert_room.size.x + vert_room.size.y,
            true
        );

        assert_eq!(right_child.position.y >= vert_room.position.y, true);
        assert_eq!(
            right_child.size.x + right_child.size.y <= vert_room.size.x + vert_room.size.y,
            true
        );
    }
    // ==============================
}
