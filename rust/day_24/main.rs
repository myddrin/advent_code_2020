use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Direction {
    fn from_string(value: &str) -> Option<Direction> {
        use Direction::*;
        match value {
            "e" => Some(E),
            "se" => Some(SE),
            "sw" => Some(SW),
            "w" => Some(W),
            "nw" => Some(NW),
            "ne" => Some(NE),
            _ => None
        }
    }

    fn vector_from_string(values: String) -> Vec<Direction> {
        let mut current = String::new();
        let mut rv = Vec::new();

        for c in values.chars() {
            current += &c.to_string();
            if let Some(d) = Self::from_string(&current) {
                rv.push(d);
                current.clear();
            }
        }

        rv
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self {x, y}
    }

    fn next(&self, dir: &Direction) -> Position {
        use Direction::*;
        match dir {
            NW => Self::new(self.x - 1, self.y + 1),
            NE => Self::new(self.x, self.y + 1),
            W => Self::new(self.x - 1, self.y),
            E => Self::new(self.x + 1, self.y),
            SW => Self::new(self.x, self.y - 1),
            SE => Self::new(self.x + 1, self.y - 1),
        }
    }

    fn follow(&self, directions: &[Direction]) -> Position {
        let mut current = self.clone();
        for d in directions {
            let n = current.next(d);
            // println!("{:?} + {:?} => {:?}", current, d, n);
            current = n;
        }
        current
    }

    fn neighbours(&self) -> Vec<Position> {
        use Direction::*;
        [NW, NE, W, E, SW, SE].iter().map(|d|self.next(d)).collect()
    }
}

#[derive(Clone)]
struct Map {
    map: HashMap<Position, bool>,  // false = white, true = black
}

impl Map {
    fn create_map(directions: &[Vec<Direction>]) -> Self {
        let mut map = HashMap::new();
        let root = Position::new(0, 0);

        for d in directions {
            let p = root.follow(&d);
            let v = *map.get(&p).unwrap_or(&false);
            map.insert(p, !v);
        }

        Self {
            map
        }
    }

    fn count(&self, black: bool) -> usize {
        self.map.values().filter(|&v| *v == black).count()
    }

    fn black_neighbours(&self, position: &Position) -> usize {
        position.neighbours().iter()
            .map(|p| {
                let v = *self.map.get(&p).unwrap_or(&false);
                if v {
                    1
                } else {
                    0
                }
            }).sum()
    }

    fn next(&self) -> Map {
        let mut to_consider = HashMap::new();
        for p in self.map.iter().filter(|&(_, v)| *v).map(|(p, _)| p) {
            if !to_consider.contains_key(p) {
                to_consider.insert(p.clone(), self.black_neighbours(&p));
            }
            for n in p.neighbours() {
                if !to_consider.contains_key(&n) {
                    to_consider.insert(n.clone(), self.black_neighbours(&n));
                }
            }
        }
        // println!("Consider {} positions", to_consider.len());

        let mut rv = Map {
            map: self.map.clone()
        };
        for (p, _) in to_consider.iter().filter(|&(p, n)| {
            // Any black tile with zero or more than 2 adjacent black tiles
            let v = *self.map.get(p).unwrap_or(&false);
            // println!("Considering {:?} {} with {} black neighbours",
            //     p,
            //     if v {"black"} else {"white"},
            //     n,
            // );
            (v && (*n == 0 || *n > 2))
                ||
            // Any white tile with exactly 2 adjacent black tiles
            (!v && *n == 2)
        }) {
            let v = *self.map.get(p).unwrap_or(&false);
            // println!("  {:?} => {}", p, !v);
            rv.map.insert(p.clone(), !v);
        }
        rv
    }

    fn run_for(&self, days: usize) -> Map {
        let mut current = self.clone();
        let mut d = 1;
        while d <= days {
            current = current.next();
            if d % 10 == 0 {
                println!("Day {}: {}", d, current.count(true));
            }
            d += 1;
        }
        current
    }
}

fn read(path: &str) -> io::Result<Vec<Vec<Direction>>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(Direction::vector_from_string(line));
    }
    Ok(rv)
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let map = Map::create_map(&contents);
    println!("Q1: found {} black tiles", map.count(true));

    let map = map.run_for(100);
    println!("Q2: found {} black tiles after 100 days", map.count(true));
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;
    use rstest::rstest;

    #[rstest(directions, exp_position,
    // the basics
    case(vec!(E), Position::new(1, 0)),
    case(vec!(W), Position::new(-1, 0)),
    case(vec!(NW), Position::new(-1, 1)),
    case(vec!(NE), Position::new(0, 1)),
    case(vec!(SW), Position::new(0, -1)),
    case(vec!(SE), Position::new(1, -1)),
    // return
    case(vec!(E, W), Position::new(0, 0)),
    case(vec!(W, E), Position::new(0, 0)),
    case(vec!(NW, SE), Position::new(0, 0)),
    case(vec!(NE, SW), Position::new(0, 0)),
    case(vec!(SW, NE), Position::new(0, 0)),
    case(vec!(SE, NW), Position::new(0, 0)),
    )]
    fn test_follow(directions: Vec<Direction>, exp_position: Position) {
        let root = Position::new(0, 0);
        assert_eq!(root.follow(&directions), exp_position);
    }

    #[rstest(path, exp_colour, exp_count,
    case("day_24/test_1.txt", true, 10),
    case("day_24/test_1.txt", false, 5),
    case("day_24/input.txt", true, 300),
    )]
    fn test_create_map(path: &str, exp_colour: bool, exp_count: usize) {
        let contents = read(&path).expect("no content");
        let map = Map::create_map(&contents);
        assert_eq!(map.count(exp_colour), exp_count);
    }

    #[rstest(path, pos, exp_neighbours,
    case("day_24/test_1.txt", Position::new(-1, 0), 5),
    )]
    fn test_neighbours(path: &str, pos: Position, exp_neighbours: usize) {
        let contents = read(&path).expect("no content");
        let map = Map::create_map(&contents);
        assert_eq!(map.black_neighbours(&pos), exp_neighbours);
    }

    #[rstest(path, days, exp_colour, exp_count,
    case("day_24/test_1.txt", 1, true, 15),
    case("day_24/test_1.txt", 100, true, 2208),
    case("day_24/input.txt", 100, true, 3466),
    )]
    fn test_run_for(path: &str, days: usize, exp_colour: bool, exp_count: usize) {
        let contents = read(&path).expect("no content");
        let map = Map::create_map(&contents).run_for(days);
        assert_eq!(map.count(exp_colour), exp_count);
    }
}
