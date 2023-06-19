use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Group {
    size: usize,
    answers: HashMap<char, usize>,  // question => nb answers
}

impl Group {
    fn read(path: &str) -> io::Result<Vec<Group>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        let mut current_group = Group {
            size: 0,
            answers: HashMap::new(),
        };
        for line in br.lines() {
            let line = line?;
            if line.is_empty() {
                // group is finished
                rv.push(current_group.clone());
                current_group.clear();
            } else {
                current_group.size += 1;
                for c in line.chars() {
                    current_group.insert(&c);
                }
            }
        }
        rv.push(current_group);
        Ok(rv)
    }

    fn insert(&mut self, question: &char) -> usize {
        let v = self.answers.get(question).unwrap_or(&0) + 1;
        self.answers.insert(question.clone(), v);
        v
    }

    fn clear(&mut self) {
        self.size = 0;
        self.answers.clear();
    }

    fn sum_anyone(&self) -> usize {
        self.answers.len()
    }

    fn sum_everyone(&self) -> usize {
        self.answers.iter().filter(|&(_k, v)| *v == self.size).count()
    }
}

fn groups_sums_anyone(data: &[Group]) -> usize {
    data.iter().map(|h| h.sum_anyone()).sum()
}

fn groups_sums_everyone(data: &[Group]) -> usize {
    data.iter().map(|h| h.sum_everyone()).sum()
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Group::read(&path).expect("no content");

    println!("Found {} groups", contents.len());
    println!("Q1: anyone answered yes: {}", groups_sums_anyone(&contents));
    println!("Q1: everyone answered yes: {}", groups_sums_everyone(&contents));
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_size, exp_sum,
    case(&"day_06/test_1.txt", 5, 11),
    case(&"day_06/input.txt", 490, 6735),
    )]
    fn test_anyone_sum(path: &str, exp_size: usize, exp_sum: usize) {
        let rv = Group::read(path);
        assert!(rv.is_ok());
        let rv = rv.unwrap();
        assert_eq!(rv.len(), exp_size);
        assert_eq!(groups_sums_anyone(&rv), exp_sum);
    }

    #[rstest(path, exp_size, exp_sum,
    case(&"day_06/test_1.txt", 5, 6),
    case(&"day_06/input.txt", 490, 3221),
    )]
    fn test_everyone_sum(path: &str, exp_size: usize, exp_sum: usize) {
        let rv = Group::read(path);
        assert!(rv.is_ok());
        let rv = rv.unwrap();
        assert_eq!(rv.len(), exp_size);
        assert_eq!(groups_sums_everyone(&rv), exp_sum);
    }
}
