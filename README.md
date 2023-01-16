# 2D Map Generation Algorithms

## Binary Search Partition (BSP)
Recursively divides the map into rooms.

Example:
```
$ cargo run --example bsp-map
```

Code example:
```rust
BSPMap::new(Point::new(25,25), SeedableRng::seed_from_u64(124));
```

### Features   
- Custom size (Minimum size of 20,20)
- Seedable rng

### Credits
Credit to https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268 and https://github.com/whostolemyhat/dungeon
for algorithm and rust implementation help.