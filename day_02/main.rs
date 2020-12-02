use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read(path: &str) -> io::Result<Vec<PwdEntry>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(PwdEntry::from_string(line));
    }
    Ok(rv)
}

#[derive(Debug)]
struct PwdEntry {
    value_a: usize,  // Q1: min, Q2: index (1 base)
    value_b: usize,  // Q1: max, Q2: index (1 base)
    letter: char,
    password: String,
}

impl PwdEntry {
    fn from_string(line: String) -> PwdEntry {
        let line = line
            .replace(":", "")
            .replace("-", " ");
        let fields: Vec<&str> = line.split_whitespace().collect();
        PwdEntry{
            value_a: fields[0].parse().unwrap(),
            value_b: fields[1].parse().unwrap(),
            letter: fields[2].to_string().chars().next().unwrap(),
            password: fields[3].to_string(),
        }
    }

    fn q1_is_valid(&self) -> bool {
        let count = self.password.chars()
            .filter(|&c| c == self.letter).count();
        // eprintln!("Found {} {} in {:?}", count, self.letter, self);
        self.value_a <= count && self.value_b >= count
    }

    fn q2_is_valid(&self) -> bool {
        let letters: Vec<char> = self.password.chars().collect();
        let mut found = 0;
        if *letters.get(self.value_a - 1).unwrap_or(&' ') == self.letter {
            found += 1;
        }
        if *letters.get(self.value_b - 1).unwrap_or(&' ') == self.letter {
            found += 1;
        }
        // eprintln!("Found {} {} times at positions {:?}", self.letter, found, self);
        return found == 1
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let mut valid_q1 = 0;
    let mut valid_q2 = 0;
    for r in &contents {
        if r.q1_is_valid() {
            valid_q1 += 1;
        }
        if r.q2_is_valid() {
            valid_q2 += 1;
        }
    }

    println!("Q1: {}/{} rules are valid", valid_q1, contents.len());
    println!("Q2: {}/{} rules are valid", valid_q2, contents.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input,
    case(&"1-3 a: abcde"),
    case(&"2-9 c: ccccccccc"),
    )]
    fn test_q1_valid(input: &str) {
        assert!(PwdEntry::from_string(input.to_string()).q1_is_valid());
    }

    #[rstest(input,
    case(&"1-3 b: cdefg"),
    )]
    fn test_q1_invalid(input: &str) {
        assert!(!PwdEntry::from_string(input.to_string()).q1_is_valid());
    }

    #[rstest(input,
    case(&"1-3 a: abcde"),

    )]
    fn test_q2_valid(input: &str) {
        assert!(PwdEntry::from_string(input.to_string()).q2_is_valid());
    }

    #[rstest(input,
    case(&"1-3 b: cdefg"),
    case(&"2-9 c: ccccccccc"),
    )]
    fn test_q2_invalid(input: &str) {
        assert!(!PwdEntry::from_string(input.to_string()).q2_is_valid());
    }
}
