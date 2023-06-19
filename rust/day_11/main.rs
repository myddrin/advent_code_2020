use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;


#[derive(Debug,Clone,PartialEq,Eq,Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Position {
        Position {x, y}
    }

    fn all_directions() -> Vec<Position> {
        vec!(
            Position::new(1, 0),
            Position::new(-1, 0),
            Position::new(0, 1),
            Position::new(0, -1),
            Position::new(1, 1),
            Position::new(1, -1),
            Position::new(-1, 1),
            Position::new(-1, -1),
        )
    }

    fn adjacent(&self, other: &Self) -> bool {
        (self.x - other.x).abs() <= 1
            && (self.y - other.y).abs() <= 1
    }

    fn next(&self, direction: &Self) -> Position {
        Position {
            x: self.x + direction.x,
            y: self.y + direction.y,
        }
    }
}

#[derive(Debug,Clone)]
struct Seat {
    position: Position,
    empty: bool
}

impl Seat {
    fn read(path: &str) -> io::Result<Vec<Seat>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for (y, line) in br.lines().enumerate() {
            let line = line?;
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;  // skip the floor.
                }
                let empty= c == 'L';
                rv.push(Seat::new(x as i32, y as i32, empty));
            }
        }
        Ok(rv)
    }

    fn new(x: i32, y: i32, empty: bool) -> Seat {
        Seat {
            position: Position{x, y},
            empty,
        }
    }

    fn adjacent(&self, other: &Self) -> bool {
        self.position != other.position && self.position.adjacent(&other.position)
    }

    fn new_state(&self, neighbours: &[&Seat], max_neighbours: usize) -> bool {
        let occupied_neighbours = neighbours.iter()
            .filter(|&n|!n.empty)
            .count();
        if self.empty {
            return occupied_neighbours != 0  // occupied if there are no neighbours
        }
        if occupied_neighbours >= max_neighbours {
            true  // too many neighbours, leave
        } else {
            self.empty  // stay the same
        }
    }

    fn build_neighbours_q1(seat_map: &HashMap<Position, Seat>) -> HashMap<Position, Vec<Position>> {
        let mut neighbours = HashMap::new();
        // build neighbours, not efficient but I don't want to create a position map...
        for s in seat_map.values() {
            let n: Vec<Position> = seat_map.values()
                .filter(|&o| s.adjacent(o))
                .map(|o| o.position.clone())
                .collect();
            // println!("{:?} has {} neighbours: {:?}", s, neighbours.len(), neighbours);
            assert!(n.len() <= 8);
            neighbours.insert(s.position.clone(), n);
        }
        neighbours
    }

    fn first_visible(from: &Position, dir: &Position, seat_map: &HashMap<Position, Seat>) -> Option<Position> {
        let rows = seat_map.keys().map(|p| p.y).max().unwrap() + 1;
        let cols = seat_map.keys().map(|p| p.x).max().unwrap() + 1;

        let mut c = from.next(dir);
        loop {
            if c.x < 0 || c.y < 0 || c.x >= cols || c.y >= rows {
                break;
            }
            if let Some(_s) = seat_map.get(&c) {
                return Some(c);
            }
            c = c.next(dir);
        }
        None
    }

    fn build_neighbours_q2(seat_map: &HashMap<Position, Seat>) -> HashMap<Position, Vec<Position>> {
        let mut neighbours = HashMap::new();
        // build neighbours
        for s in seat_map.values() {
            let mut n: Vec<Position> = Vec::new();
            for d in Position::all_directions() {
                if let Some(p) = Self::first_visible(&s.position, &d, seat_map) {
                    n.push(p);
                }
            }
            assert!(n.len() <= 16);
            neighbours.insert(s.position.clone(), n);
        }
        neighbours
    }

    fn neighbours<'a>(seat_map: &'a HashMap<Position, Seat>, neighbours_idx: &[Position]) -> Vec<&'a Seat> {
        neighbours_idx.iter().map(|u| &seat_map[u]).collect()
    }

    fn do_prediction(seat_map: &HashMap<Position, Seat>, neighbours_pos: &HashMap<Position, Vec<Position>>, max_neighbours: usize) -> Vec<(Position, bool)> {
        seat_map.values()
            .map(|s| (s.position.clone(), s.new_state(
                &Self::neighbours(seat_map, &neighbours_pos[&s.position]),
                max_neighbours,
            )))
            .collect()
    }

    fn predict(
        seat_map: &HashMap<Position, Seat>,
        neighbours_pos: &HashMap<Position, Vec<Position>>,
        max_neighbours: usize
    ) -> usize {
        let mut seat_map = seat_map.clone();
        let mut compute = true;
        let mut t: usize = 0;

        while compute {
            let changes = Self::do_prediction(&seat_map, &neighbours_pos, max_neighbours);

            let mut n_change = 0;
            for (p, s) in changes.iter() {
                let mut o = seat_map.get_mut(p).unwrap();
                if o.empty != *s {
                    n_change += 1;
                }
                o.empty = *s;
            }
            compute = n_change > 0;
            t += 1;
        }

        println!("Computed in {} iterations", t);
        seat_map.values().filter(|&s|!s.empty).count()
    }

    fn predict_q1(seat_map: &HashMap<Position, Seat>) -> usize {
        let neighbours_pos = Self::build_neighbours_q1(&seat_map);
        Self::predict(seat_map, &neighbours_pos, 4)
    }

    fn predict_q2(seat_map: &HashMap<Position, Seat>) -> usize {
        let neighbours_pos = Self::build_neighbours_q2(&seat_map);
        Self::predict(seat_map, &neighbours_pos, 5)
    }

    fn to_map(seats: Vec<Seat>) -> HashMap<Position, Seat> {
        let mut map :HashMap<Position, Seat> = HashMap::new();
        for s in seats {
            map.insert(s.position.clone(), s);
        }
        map
    }

    fn to_strings(seat_map: &HashMap<Position, Seat>) -> Vec<String> {
        let mut rv = Vec::new();
        let lines = seat_map.keys().map(|p|p.y).max().unwrap() + 1;
        let cols = seat_map.keys().map(|p|p.x).max().unwrap() + 1;
        for y in 0..lines {
            let mut str = String::new();
            for x in 0..cols {
                if let Some(s) = seat_map.get(&Position{x, y}) {
                    if s.empty {
                        str += "L";
                    } else {
                        str += "#";
                    }
                } else {
                    str += ".";
                }
            }
            rv.push(str);
        }
        rv
    }

    fn print(seat_map: &HashMap<Position, Seat>) {
        for l in Self::to_strings(seat_map) {
            println!("{}", l);
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Seat::read(&path).expect("no content");

    println!("Initial seats");
    let seat_map = Seat::to_map(contents);
    Seat::print(&seat_map);

    let free_seats = Seat::predict_q1(&seat_map);
    println!("Q1: Free seats: {}", free_seats);

    let free_seats = Seat::predict_q2(&seat_map);
    println!("Q2: Free seats: {}", free_seats);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_seats,
    case(&"day_11/test_1.txt", 37),
    case(&"day_11/input.txt", 2247),
    )]
    fn test_predict_q1(path: &str, exp_seats: usize) {
        let contents = Seat::read(&path);
        assert!(contents.is_ok());
        let contents = Seat::to_map(contents.unwrap());
        assert_eq!(Seat::predict_q1(&contents), exp_seats);
    }

    #[rstest(path, pos, exp_neighbours,
    case(&"day_11/test_1.txt", Position::new(0, 0), vec!(
        Position::new(2, 0),
        Position::new(1, 1),
        // Position::new(2, 2),
        Position::new(0, 1),
        // Position::new(0, 2),
    )),
    case(&"day_11/test_2.txt", Position::new(3, 4), vec!(
        Position::new(2, 4),
        Position::new(4, 5),
        Position::new(8, 4),
        Position::new(3, 8),
        Position::new(3, 1),
        Position::new(7, 0),
        Position::new(0, 7),
        Position::new(1, 2),
    )),
    case(&"day_11/test_3.txt", Position::new(1, 1), vec!(
        Position::new(3, 1),
    )),
    case(&"day_11/test_4.txt", Position::new(3, 3), Vec::new()),
    )]
    fn test_check_neighbours_q2(path: &str, pos: Position, exp_neighbours: Vec<Position>) {
        let contents = Seat::read(path);
        assert!(contents.is_ok());
        let contents = Seat::to_map(contents.unwrap());

        let neighbours = Seat::build_neighbours_q2(&contents);
        let p = neighbours.get(&pos);
        assert!(p.is_some());
        let p = p.unwrap();
        println!("Found {:?}", p);
        assert_eq!(exp_neighbours.len(), p.len());
        for p in p {
            assert!(exp_neighbours.contains(p));
        }
    }

    #[rstest(path, exp_seats,
    case(&"day_11/test_1.txt", 26),
    case(&"day_11/input.txt", 2011),
    )]
    fn test_predict_q2(path: &str, exp_seats: usize) {
        let contents = Seat::read(&path);
        assert!(contents.is_ok());
        let contents = Seat::to_map(contents.unwrap());
        assert_eq!(Seat::predict_q2(&contents), exp_seats);
    }
}
