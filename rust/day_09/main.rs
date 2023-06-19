use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn read(path: &str) -> io::Result<Vec<u64>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        if let Ok(v) = line.parse() {
            rv.push(v);
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "invalid line"));
        }
    }
    Ok(rv)
}

fn is_valid(history: &[u64], value: u64) -> bool {
    // brute force check, we may want something a bit more clever if we can.
    for (idx, a) in history.iter().enumerate() {
        for (_j, b) in history.iter().enumerate().filter(|&(i, _)| i > idx) {
            if a + b == value {
                return true;
            }
        }
    }
    false
}

fn first_invalid(values: &[u64], look_back: usize) -> Option<u64> {
    let mut history: Vec<u64> = Vec::new();

    for v in values {
        if history.len() < look_back {
            history.push(*v);
        } else {
            if !is_valid(&history, *v) {
                return Some(v.clone());
            }
            history.remove(0);
            history.push(*v);
        }
    }
    None
}

fn has_weakness(values: &[u64], invalid: u64, size: usize) -> Option<Vec<u64>> {
    let mut rv = Vec::new();

    for v in values {
        if rv.len() < size {
            rv.push(*v);
        } else {
            let sum: u64 = rv.iter().sum();
            if sum == invalid {
                return Some(rv);
            }
            rv.remove(0);
            rv.push(*v);
        }
    }
    None
}

fn search_weakness(values: &[u64], invalid: u64) -> Option<Vec<u64>> {
    eprintln!("Scanning...");
    for size in 2..values.len() {
        if size % 10 == 0 {
            eprintln!("Scanning for size {}", size);
        }
        let rv = has_weakness(values, invalid, size);
        if rv.is_some() {
            return rv;
        }
    }
    None
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let invalid = first_invalid(&contents, 25);
    if let Some(invalid) = invalid {
        println!("Q1: first invalid entry: {}", invalid);

        let weakness = search_weakness(&contents, invalid);
        if let Some(weakness) = weakness {
            let min = weakness.iter().min().unwrap();
            let max = weakness.iter().max().unwrap();
            println!("Q2: list of {} elem, smallest {} and largest {}: {}",
                weakness.len(),
                min,
                max,
                min + max
            );
        } else {
            println!("No weakness found");
        }
    } else {
        println!("Valid XMAS");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, exp,
    case(&[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,], Some(127))
    )]
    fn test_lookback_of_5(input: &[u64], exp: Option<u64>) {
        assert_eq!(first_invalid(input, 5), exp);
    }

    #[rstest(input, weakness, exp_min, exp_max,
    case(&[35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,], 127, 15, 47),
    )]
    fn test_search_weakness(input: &[u64], weakness: u64, exp_min: u64, exp_max: u64) {
        let rv = search_weakness(input, weakness);
        assert!(rv.is_some());
        let rv = rv.unwrap();
        assert_eq!(*rv.iter().min().unwrap(), exp_min);
        assert_eq!(*rv.iter().max().unwrap(), exp_max);
    }
}
