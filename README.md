# 2D Map Generation Algorithms

## Binary Search Partition (BSP)
Recursively divides the map into rooms.

Example:
```
$ cargo run --example bsp-map
```

Code example:
```rust
// BSPMap::new(position, size, min_room_size, max_room_size)
BSPMap::new(Point::new(20,50), SeedableRng::seed_from_u64(1), Point::new(2,5), Point::new(10,15))
```

### Features   
- Custom size (Minimum size of 20,20)
- Seedable rng

### Credits
Credit to https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268 and https://github.com/whostolemyhat/dungeon
for algorithm and rust implementation help.