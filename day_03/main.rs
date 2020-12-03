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

    fn start_position(&self) -> Position {
        Position{x: 0, y: 0}
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
        let mut tree_count = 0;
        let mut current_position = self.start_position();
        while current_position.y < self.height() {
            if self.is_tree_at(&current_position) {
                tree_count += 1;
            }

            current_position = current_position.next_position(slope);
        }
        tree_count
    }
}

#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position{x, y}
    }

    fn next_position(&self, slope: &Position) -> Position {
        Position::new(self.x + slope.x, self.y + slope.y)
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let tree_map = Map::read(&path).expect("no content");

    let slope = Position{x: 3, y: 1};
    let tree_count = tree_map.count_trees_on_slope(&slope);

    println!("Q1: Found {} trees using slope {:?}", tree_count, slope);

    let mut mult = 1;  // we know that we'll find at least 1 tree
    for slope in vec!(
        Position::new(1, 1),
        Position::new(3, 1),
        Position::new(5, 1),
        Position::new(7, 1),
        Position::new(1, 2),
    ) {
        let trees = tree_map.count_trees_on_slope(&slope);
        println!("Found {} trees using slope {:?}", trees, slope);
        mult *= trees;
    }
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
}
