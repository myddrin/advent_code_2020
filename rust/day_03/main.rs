use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug)]
struct Map {
    trees: Vec<Vec<bool>>,
}

impl Map {
    fn from_string(line: String) -> Vec<bool> {
        line.chars().map(|c| c == '#').collect()
    }

    fn read(path: &str) -> io::Result<Map> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Map{trees: Vec::new()};

        for line in br.lines() {
            let line = line?;
            rv.trees.push(Self::from_string(line));
        }
        Ok(rv)
    }

    fn new_slope(&self, slope: &Position) -> Slope {
        Slope{
            slope: slope.clone(),
            current_position: Position::new(0, 0),
            max_height: self.height(),
        }
    }

    fn width(&self) -> usize {
        self.trees[0].len()
    }

    fn height(&self) -> usize {
        self.trees.len()
    }

    fn is_tree_at(&self, position: &Position) -> bool {
        self.trees[position.y][position.x % self.width()]
    }

    fn count_trees_on_slope(&self, slope: &Position) -> usize {
        self.new_slope(slope)
            .filter(|p| self.is_tree_at(p))
            .count()
    }

    fn mult_trees_on_slopes(&self, slopes: &[Position]) -> usize {
        let mut mult = 1;
        for slope in slopes {
            let trees = self.count_trees_on_slope(slope);
            // println!("Found {} trees using slope {:?}", trees, slope);
            mult *= trees;
        }
        mult
    }
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position{x, y}
    }
}

#[derive(Debug)]
struct Slope {
    slope: Position,
    current_position: Position,
    max_height: usize,
}

impl Iterator for Slope {
    type Item = Position;

    fn next(&mut self) -> Option<Position> {
        self.current_position.x += self.slope.x;
        self.current_position.y += self.slope.y;
        if self.current_position.y < self.max_height {
            Some(self.current_position.clone())
        } else {
            None
        }
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let tree_map = Map::read(&path).expect("no content");

    let slope = Position{x: 3, y: 1};
    let tree_count = tree_map.count_trees_on_slope(&slope);
    println!("Q1: Found {} trees using slope {:?}", tree_count, slope);

    let mult = tree_map.mult_trees_on_slopes(&[
        Position::new(1, 1),
        Position::new(3, 1),
        Position::new(5, 1),
        Position::new(7, 1),
        Position::new(1, 2),
    ]);
    println!("Q2: mult of trees is {}", mult);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, map_line,
    case(&"..##.......", vec!(false, false, true, true, false, false, false, false, false, false, false))
    )]
    fn test_from_line(input: &str, map_line: Vec<bool>) {
        assert_eq!(Map::from_string(input.to_string()), map_line)
    }

    #[rstest(file, slope, exp_trees,
    case(&"day_03/test_1.txt", Position{x: 3, y: 1}, 7),
    )]
    fn test_count_trees(file: &str, slope: Position, exp_trees: usize) {
        let tree_map = Map::read(file);
        assert!(tree_map.is_ok());
        let tree_map = tree_map.unwrap();
        assert_eq!(tree_map.count_trees_on_slope(&slope), exp_trees);
    }

    #[rstest(file, slopes, exp_trees,
    case(
        &"day_03/test_1.txt",
        &[
            Position::new(1, 1),
            Position::new(3, 1),
            Position::new(5, 1),
            Position::new(7, 1),
            Position::new(1, 2),
        ],
        336,
    )
    )]
    fn test_mult_trees(file: &str, slopes: &[Position], exp_trees: usize) {
        let tree_map = Map::read(file);
        assert!(tree_map.is_ok());
        let tree_map = tree_map.unwrap();
        assert_eq!(tree_map.mult_trees_on_slopes(slopes), exp_trees);
    }
}
