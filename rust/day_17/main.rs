use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone,Debug,Hash, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Position {
    fn new(x: i32, y: i32, z: i32, w: i32) -> Position {
        Position{x, y, z, w}
    }

    fn neighbours(&self, use_4d: bool) -> Vec<Position> {
        let mut rv = Vec::new();
        if use_4d {
            for dz in &[-1, 0, 1] {
                for dy in &[-1, 0, 1] {
                    for dx in &[-1, 0, 1] {
                        for dw in &[-1, 0, 1] {
                            if *dz == 0 && *dy == 0 && *dx == 0 && *dw == 0 {
                                continue;
                            }
                            rv.push(Position::new(self.x + dx, self.y + dy, self.z + dz, self.w + dw));
                        }
                    }
                }
            }
        } else {
            for dz in &[-1, 0, 1] {
                for dy in &[-1, 0, 1] {
                    for dx in &[-1, 0, 1] {
                        if *dz == 0 && *dy == 0 && *dx == 0 {
                            continue;
                        }
                        rv.push(Position::new(self.x + dx, self.y + dy, self.z + dz, 0));
                    }
                }
            }
        }
        rv
    }
}

#[derive(Clone)]
struct Space {
    cubes: HashMap<Position, bool>,
}

impl Space {
    fn read(path: &str) -> io::Result<Space> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = HashMap::new();

        let z = 0;
        for (y, line) in br.lines().enumerate() {
            let line = line?;
            for (x, c) in line.chars().enumerate() {
                let p = Position::new(x as i32, y as i32, z, 0);
                let active = c == '#';
                if active {
                    rv.insert(p, active);
                }
            }
        }
        Ok(Space{cubes: rv})
    }

    fn new_state(&self, position: &Position, use_4d: bool) -> bool {
        let neighbours = position.neighbours(use_4d);
        let act_neighbours = self.cubes.iter()
            .filter(|&(p, v)| neighbours.contains(p) && *v)
            .map(|(_, v)| v)
            .count();
        let current = self.cubes.get(position).unwrap_or(&false);
        if *current {
            // if active, stay active if 2 or 3 neighbours are active
            act_neighbours == 2 || act_neighbours == 3
        } else {
            // if inactive becomes active if 3 neighbours are active
            act_neighbours == 3
        }
    }

    fn simulate(&mut self, use_4d: bool) {
        let mut new_cubes = HashMap::new();

        for (p, _) in &self.cubes {
            new_cubes.insert(p.clone(), self.new_state(&p, use_4d));

            for n in p.neighbours(use_4d) {
                if !new_cubes.contains_key(&n) {
                    new_cubes.insert(n.clone(), self.new_state(&n, use_4d));
                }
            }
        }
        // println!("Created {} cubes to check from {} cubes", new_cubes.len(), self.cubes.len());
        self.cubes.clear();
        for (p, v) in new_cubes {
            if v {
                self.cubes.insert(p, v);
            }
        }
    }

    fn run(content: &Self, turns: usize, use_4d: bool) -> Self {
        let mut space = content.clone();
        for _t in 0..turns {
            // println!("t={} active_cubes={}", t, space.cubes.len());
            space.simulate(use_4d);
        }
        space
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Space::read(&path).expect("no content");

    let start = Instant::now();
    let space = Space::run(&contents, 6, false);
    let duration = start.elapsed();
    println!("Q1: {} active cubes after 6 turns ({:?})", space.cubes.len(), duration);
    println!();

    let start = Instant::now();
    let space = Space::run(&contents, 6, true);
    let duration = start.elapsed();
    println!("Q2: {} active hypercubes after 6 turns ({:?})", space.cubes.len(), duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, use_4d, exp,
    case(&"day_17/test_1.txt", false, 112),
    case(&"day_17/input.txt", false, 315),
    case(&"day_17/test_1.txt", true, 848),
    case(&"day_17/input.txt", true, 1520),  // only run in release, otherwise it >1min
    )]
    fn test_run(path: &str, use_4d: bool, exp: usize) {
        let contents = Space::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        let space = Space::run(&contents, 6, use_4d);
        assert_eq!(space.cubes.len(), exp);
    }
}
