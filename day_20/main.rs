use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Clone,Hash,Eq,PartialEq,Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    fn neighbours(&self) -> Vec<Position> {
        vec!(
            Position::new(self.x - 1, self.y),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y - 1),
            Position::new(self.x, self.y + 1),
        )
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    data: HashMap<Position, bool>,
    rot: usize,
    flip: (bool, bool),
}

impl Tile {
    fn new() -> Self {
        Self {
            id: 0,
            data: HashMap::new(),
            rot: 0,
            flip: (false, false),
        }
    }

    fn read(path: &str) -> io::Result<Vec<Tile>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();
        let mut current_tile = Tile::new();
        let mut current_row = 0;

        for line in br.lines() {
            let line = line?;

            if line.is_empty() {
                rv.push(current_tile);
                current_tile = Tile::new();
            } else if line.starts_with("Tile ") {
                let id = line
                    .replace("Tile ", "")
                    .replace(":", "")
                    .parse().unwrap();
                current_tile.id = id;
                // println!("Loading tile {}", current_tile.id);
                current_row = 0;
            } else {
                for (x, d) in line.chars().enumerate() {
                    let p = Position::new(x as i32, current_row);
                    current_tile.data.insert(p, d == '#');
                }
                current_row += 1;
            }
        }
        if !current_tile.data.is_empty() {
            // in case the last line is not an empty line.
            rv.push(current_tile);
        }
        Ok(rv)
    }

    fn get_line(&self, y: i32) -> String {
        let mut rv = String::new();
        // we know we store from 0
        for x in 0..self.width() {
            let d = self.data.get(&Position::new(x, y)).unwrap();
            if *d {
                rv += "#";
            } else {
                rv += ".";
            }
        }
        rv
    }

    fn get_col(&self, x: i32) -> String {
        let mut rv = String::new();
        // we know we store from 0
        for y in 0..self.height() {
            let d = self.data.get(&Position::new(x, y)).unwrap();
            if *d {
                rv += "#";
            } else {
                rv += ".";
            }
        }
        rv
    }

    fn name(&self) -> String {
        format!("{}.{}/{:?}", self.id, self.rot, self.flip)
    }

    fn rotate(&self) -> Tile {
        let mut new_data = HashMap::new();

        let height = self.height();
        for (k, v) in self.data.iter() {
            // because y is inversed (y=1 is line 1 bellow line 0)
            let new_k = Position::new(
                height - k.y - 1,
                k.x,
            );
            new_data.insert(new_k, *v);
        }

        Tile {
            id: self.id,
            data: new_data,
            rot: self.rot + 90,
            flip: self.flip.clone(),
        }
    }

    fn flip(&self, flip_x: bool) -> Tile {
        let mut new_data = HashMap::new();
        let width = self.width();
        let height = self.height();
        for (k, v) in self.data.iter() {
            // because y is inversed (y=1 is line 1 bellow line 0)
            let new_k = if flip_x {
                Position::new(width - k.x - 1, k.y)
            } else {
                Position::new(k.x, height - k.y - 1)
            };
            new_data.insert(new_k, *v);
        }

        let new_flip = if flip_x {
            (!self.flip.0, self.flip.1)
        } else {
            (self.flip.0, !self.flip.1)
        };
        Tile {
            id: self.id,
            data: new_data,
            rot: self.rot,
            flip: new_flip,
        }
    }

    fn next(&self) -> Option<Tile> {
        if self.rot < 360 {
            return Some(self.rotate());
        }
        // we tried all rotations, let's flip
        let mut rv = if !self.flip.0 && !self.flip.1 {
            // we've done no flipping, try flipping x
            self.flip(true)
        } else if self.flip.0 && !self.flip.1 {
            // we flipped x before, flip y now => x & y are flipped
            self.flip(false)
        } else if self.flip.0 && self.flip.1 {
            // we flipped x & y, let's try with only y
            self.flip(true)
        } else {
            return None
        };

        rv.rot = rv.rot % 360;
        Some(rv)
    }

    fn width(&self) -> i32 {
        self.data.keys().map(|k| k.x).max().unwrap_or(0) + 1
    }

    fn height(&self) -> i32 {
        self.data.keys().map(|k| k.y).max().unwrap_or(0) + 1
    }
}

struct Group {
    tiles: HashMap<Position, Tile>
}

impl Group {
    fn new(first_tile: Tile) -> Self {
        let mut tiles = HashMap::new();
        // println!("First tile: {}", first_tile.name());
        // for y in 0..first_tile.height() {
        //     println!("{}", first_tile.get_line(y));
        // }
        tiles.insert(Position::new(0, 0), first_tile);
        Self {
            tiles
        }
    }

    fn is_valid_addition(&self, tile: &Tile) -> Option<Position> {
        for (p, t) in self.tiles.iter() {
            for n in p.neighbours() {
                if self.tiles.get(&n).is_none() {
                    // no data, this slot is free!
                    let cmp = if n.x < p.x {
                        // left
                        (
                            tile.get_col(tile.width() - 1),
                            t.get_col(0),
                        )
                    } else if n.x > p.x {
                        // right
                        (
                            tile.get_col(0),
                            t.get_col(t.width() - 1),
                        )
                    } else if n.y < p.y {
                        // above
                        (
                            tile.get_line(tile.height() - 1),
                            t.get_line(0),
                        )
                    } else {
                        // bellow
                        (
                            tile.get_line(0),
                            t.get_line(t.height() - 1),
                        )
                    };
                    // println!("{} next to {} in {:?}? {} == {}", tile.id, t.id, n, cmp.0, cmp.1);
                    // debug
                    if tile.id == 3079 && (t.id == 2311 || t.id == 2473) {
                        println!("{} {:?} next to {} {:?}? {} == {}",
                                 tile.name(),
                                 n,
                                 t.name(),
                                 p,
                                 cmp.0,
                                 cmp.1,
                        );
                    }

                    if cmp.0 == cmp.1 {
                        return Some(n);
                    }
                }
            }
        }
        None
    }

    fn consume_tiles(&mut self, tiles: Vec<Tile>) -> Vec<Tile> {
        let mut rv = Vec::new();
        for t in tiles {
            let mut current = Some(t.clone());
            loop {
                let current_tile = if let Some(current_tile) = current {
                    current_tile
                } else {
                    rv.push(t);
                    break;
                };
                if let Some(p) = self.is_valid_addition(&current_tile) {
                    // println!("Tile {} fits in {:?}", t.name(), p);
                    self.tiles.insert(p, current_tile.clone());
                    break;
                }
                current = current_tile.next();
            }
        }
        rv
    }

    fn merges(&mut self, tiles: &[Tile]) {
        let mut i: usize = 0;
        let mut last_size = tiles.len();
        let mut tiles_left = tiles.to_vec();

        while !tiles_left.is_empty() {
            tiles_left = self.consume_tiles(tiles_left);
            // println!("i={} merged {} tiles, {} are left", i, last_size - tiles_left.len(), tiles_left.len());
            if last_size == tiles_left.len() {
                let left: Vec<String> = tiles_left.iter().map(Tile::name).collect();
                eprintln!("deadlock i={} left={:?}", i, left);
                return;
            }
            last_size = tiles_left.len();
            i += 1;
        }
    }

    fn start_x(&self) -> i32 {
        self.tiles.keys().map(|k| k.x).min().unwrap_or(0)
    }

    fn start_y(&self) -> i32 {
        self.tiles.keys().map(|k| k.y).min().unwrap_or(0)
    }

    fn end_x(&self) -> i32 {
        self.tiles.keys().map(|k| k.x).max().unwrap_or(0)
    }

    fn end_y(&self) -> i32 {
        self.tiles.keys().map(|k| k.y).max().unwrap_or(0)
    }

    fn validation(&self) -> Vec<Vec<usize>> {
        let mut rv = Vec::new();
        for y in self.start_y()..self.end_y() + 1 {
            let mut line = Vec::new();
            for x in self.start_x()..self.end_x() + 1 {
                // there should not be gaps... but just in case?
                if let Some(t) = self.tiles.get(&Position::new(x, y)) {
                    line.push(t.id);
                }
            }
            rv.push(line);
        }
        rv
    }

    fn checksum(&self) -> usize {
        let v = self.validation();
        let top = v.first().unwrap();
        let bottom = v.last().unwrap();
        top.first().unwrap()
            * top.last().unwrap()
            * bottom.first().unwrap()
            * bottom.last().unwrap()
    }

    fn image(&self) -> Vec<String> {
        let mut rv = Vec::new();
        let ref_t = self.tiles.get(&Position::new(0, 0)).unwrap();
        let mut missing = " ".to_string();
        for _ in 0..ref_t.width() {
            missing += "?";
        }

        for y in self.start_y()..self.end_y() + 1 {
            for t_y in 0..ref_t.height() {
                let mut line = String::new();
                for x in self.start_x()..self.end_x() + 1 {
                    let pos = Position::new(x, y);
                    if let Some(p) = self.tiles.get(&pos) {
                        line += &format!(" {}", p.get_line(t_y));
                    } else {
                        if t_y == -1 {
                            line += "? ";
                        } else {
                            line += &missing;
                        }
                    }
                }
                rv.push(line);
            }
            // rv.push(String::new());
        }
        rv
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Tile::read(&path).expect("no content");
    println!("Loaded {} images", contents.len());

    let mut picture = Group::new(contents[0].clone());
    picture.merges(&contents[1..]);

    for l in picture.image() {
        println!("{}", l);
    }

    println!("Q1: checksum {}", picture.checksum());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path_a, path_b,
    case("day_20/test_2_0.txt", "day_20/test_2_90.txt"),
    case("day_20/test_2_90.txt", "day_20/test_2_180.txt"),
    case("day_20/test_2_180.txt", "day_20/test_2_270.txt"),
    case("day_20/test_2_270.txt", "day_20/test_2_0.txt"),
    )]
    fn test_rotation(path_a: &str, path_b: &str) {
        let contents_a = Tile::read(&path_a).expect("path a");
        let contents_b = Tile::read(&path_b).expect("path b");
        println!("{} -> {}", path_a, path_b);
        for (i, t) in contents_a.iter().enumerate() {
            println!("checking tile {}", t.id);
            // let original_data = t.data.clone();
            for y in 0..t.height() {
                println!("{:2} {}", y, t.get_line(y));
            }
            println!("rotates to");
            let t = t.rotate();
            for y in 0..t.height() {
                println!("{:2} {}", y, t.get_line(y));
            }
            println!("expects");
            for y in 0..contents_b[i].height() {
                println!("{:2} {}", y, contents_b[i].get_line(y));
            }
            assert_eq!(t.data, contents_b[i].data);
        }
    }

    #[rstest(path_a, path_b, flip_x,
    case("day_20/test_2_0.txt", "day_20/test_2_0x.txt", true),
    case("day_20/test_2_0.txt", "day_20/test_2_0y.txt", false),
    )]
    fn test_flip(path_a: &str, path_b: &str, flip_x: bool) {
        let contents_a = Tile::read(&path_a).expect("path a");
        let contents_b = Tile::read(&path_b).expect("path b");
        println!("{} -> {}", path_a, path_b);
        for (i, t) in contents_a.iter().enumerate() {
            println!("checking tile {}", t.id);
            // let original_data = t.data.clone();
            for y in 0..t.height() {
                println!("{}:{}", y, t.get_line(y));
            }
            println!("flip on {}", if flip_x {"x"} else {"y"});
            let t = t.flip(flip_x);
            for y in 0..t.height() {
                println!("{}:{}", y, t.get_line(y));
            }
            assert_eq!(t.data, contents_b[i].data);
        }
    }

    #[rstest(path, exp_ids,
    case("day_20/test_1.txt", vec!(
    // website examples is flipped for me
        vec!(2971, 1489, 1171),
        vec!(2729, 1427, 2473),
        vec!(1951, 2311, 3079),
    )),
    )]
    fn test_merge_data(path: &str, exp_ids: Vec<Vec<usize>>) {
        let contents = Tile::read(&path).expect("no content");
        let mut picture = Group::new(contents[0].clone());
        picture.merges(&contents[1..]);

        for l in picture.image() {
            println!("{}", l);
        }

        assert_eq!(picture.validation(), exp_ids);
    }

    #[rstest(path, exp_checksum,
    case("day_20/test_1.txt", 20899048083289),
    case("day_20/input.txt", 18482479935793),
    )]
    fn test_checksum(path: &str, exp_checksum: usize) {
        let contents = Tile::read(&path).expect("no content");
        let mut picture = Group::new(contents[0].clone());
        picture.merges(&contents[1..]);
        assert_eq!(picture.checksum(), exp_checksum);
    }
}
