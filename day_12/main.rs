use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Direction {
    N,
    S,
    E,
    W,
    L,  // left
    R,  // right
    F,  // forward
}

impl Direction {
    fn from_line(line: &str) -> Option<Direction> {
        use Direction::*;
        let value = &line[..1];
        match value {
            "N" => Some(N),
            "S" => Some(S),
            "E" => Some(E),
            "W" => Some(W),
            "L" => Some(L),
            "R" => Some(R),
            "F" => Some(F),
            _ => None,
        }
    }

    // changing direction, assume self is N, S, E, W
    fn apply(&self, action: &Action) -> Direction {
        use Direction::*;
        let other = action.action;
        let order = [N, E, S, W];
        if order.contains(&other) || other == F {
            return *self;
        }
        let index: Vec<usize> = order.iter().enumerate()
            .filter(|&(_, a)| a == self)
            .map(|(i, _)| i)
            .collect();
        let n = action.value / 90;
        let index = if other == L {
            // + len to ensure it stays positive
            (index[0] + order.len() - n as usize) % order.len()
        } else {
            (index[0] + n as usize) % order.len()
        };
        if index as usize >= order.len() {
            eprintln!("Index of {:?} is {:?}?", self, index);
        }
        order[index]
    }
}

#[derive(Debug)]
struct Action {
    action: Direction,
    value: i32,
}

impl Action {
    fn from_string(line: String) -> Option<Action> {
        let act = Direction::from_line(&line)?;
        let v = (&line[1..]).parse().unwrap();
        Some(Action{action: act, value: v})
    }

    fn read(path: &str) -> io::Result<Vec<Action>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for line in br.lines() {
            let line = line?;
            let act = Self::from_string(line).unwrap();
            rv.push(act);
        }
        Ok(rv)
    }
}

#[derive(Debug)]
struct Ship {
    facing: Direction,
    north: i32,  // negative is south
    east: i32,  // negative is west
}

impl Ship {
    fn new() -> Ship {
        use Direction::*;
        Ship {
            facing: E,
            north: 0,
            east: 0,
        }
    }

    fn apply(&mut self, action: &Action) {
        use Direction::*;
        match action.action {
            N => self.north += action.value,
            S => self.north -= action.value,
            E => self.east += action.value,
            W => self.east -= action.value,
            F => {
                self.apply(&Action {action: self.facing, value: action.value});
            }
            _ => {
                // Rotation does not change position
                self.facing = self.facing.apply(&action);
            },
        }
    }

    fn follow_q1(actions: &[Action]) -> Ship {
        let mut ship = Ship::new();

        for (i, a) in actions.iter().enumerate() {
            println!("i={} ship={:?} + {:?}", i, ship, a);
            ship.apply(a);
        }

        println!("  ship now {:?}", ship);
        ship
    }

    fn follow_q2(actions: &[Action]) -> Ship {
        let mut ship = Ship::new();
        let mut waypoint = Waypoint::new();

        for (i, a) in actions.iter().enumerate() {
            println!("i={} ship={:?} wayp={:?} + {:?}", i, ship, waypoint, a);

            if a.action == Direction::F {
                println!("move {} times n={:?} e={:?}!", a.value, waypoint.north, waypoint.east);
                ship.north += waypoint.north * a.value;
                ship.east += waypoint.east * a.value;
            } else {
                waypoint.apply(a);
            }
        }

        println!("  ship now {:?}", ship);
        println!("  waypoint now {:?}", waypoint);
        ship
    }

    fn travelled(&self) -> i32 {
        self.north.abs() + self.east.abs()
    }
}

#[derive(Debug)]
struct Waypoint {
    north: i32,  // negative is south
    east: i32,  // negative is west
}

impl Waypoint {
    fn new() -> Waypoint {
        Waypoint {
            north: 1,
            east: 10,
        }
    }

    fn rotate(&mut self, direction: Direction) {
        let east = self.east;
        let north = self.north;
        if direction == Direction::L {
            self.east = north * -1;
            self.north = east;
        } else if direction == Direction::R {
            self.east = north;
            self.north = east * -1;
        }
    }

    fn apply(&mut self, action: &Action) {
        use Direction::*;
        match action.action {
            N => self.north += action.value,
            S => self.north -= action.value,
            E => self.east += action.value,
            W => self.east -= action.value,
            F => (),  // waypoint is relative to the ship
            _ => {
                // Rotate around the ship
                let mut n = action.value / 90;
                while n > 0 {
                    self.rotate(action.action);
                    n -= 1;
                }
            },
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Action::read(&path).expect("no content");

    let ship = Ship::follow_q1(&contents);
    println!("Q1 ship distance: {}", ship.travelled());

    let ship = Ship::follow_q2(&contents);
    println!("Q2 ship distance: {}", ship.travelled());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_east, exp_north,
    case(&"day_12/test_1.txt", 17, -8),
    case(&"day_12/input.txt", 403, -187),
    )]
    fn test_follow_q1(path: &str, exp_east: i32, exp_north: i32) {
        let contents = Action::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        let ship = Ship::follow_q1(&contents);
        assert_eq!(ship.east, exp_east);
        assert_eq!(ship.north, exp_north);
    }

    #[rstest(path, exp_east, exp_north,
    case(&"day_12/test_1.txt", 214, -72),
    case(&"day_12/input.txt", -29191, 12822),
    )]
    fn test_follow_q2(path: &str, exp_east: i32, exp_north: i32) {
        let contents = Action::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        let ship = Ship::follow_q2(&contents);
        assert_eq!(ship.east, exp_east);
        assert_eq!(ship.north, exp_north);
    }

    #[rstest(ship, action, exp_ship,
    case(Direction::E, Action{action: Direction::L, value: 90}, Direction::N),
    case(Direction::N, Action{action: Direction::L, value: 90}, Direction::W),
    case(Direction::W, Action{action: Direction::L, value: 90}, Direction::S),
    case(Direction::S, Action{action: Direction::L, value: 90}, Direction::E),
    case(Direction::E, Action{action: Direction::R, value: 90}, Direction::S),
    case(Direction::S, Action{action: Direction::R, value: 90}, Direction::W),
    case(Direction::W, Action{action: Direction::R, value: 90}, Direction::N),
    // the ones that do not rotate
    case(Direction::E, Action{action: Direction::N, value: 90}, Direction::E),
    case(Direction::E, Action{action: Direction::E, value: 90}, Direction::E),
    case(Direction::E, Action{action: Direction::S, value: 90}, Direction::E),
    case(Direction::E, Action{action: Direction::W, value: 90}, Direction::E),
    case(Direction::E, Action{action: Direction::F, value: 90}, Direction::E),
    // by more than 90
    case(Direction::E, Action{action: Direction::L, value: 180}, Direction::W),
    case(Direction::E, Action{action: Direction::L, value: 270}, Direction::S),
    case(Direction::E, Action{action: Direction::L, value: 360}, Direction::E),
    case(Direction::E, Action{action: Direction::R, value: 180}, Direction::W),
    case(Direction::E, Action{action: Direction::R, value: 270}, Direction::N),
    case(Direction::E, Action{action: Direction::R, value: 360}, Direction::E),
    )]
    fn test_apply_direction(ship: Direction, action: Action, exp_ship: Direction) {
        let new_ship = ship.apply(&action);
        println!("{:?} + {:?} => {:?}", ship, action, new_ship);
        assert_eq!(new_ship, exp_ship);
    }
}
