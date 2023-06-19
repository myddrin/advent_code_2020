use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead, ErrorKind};
use combinations::Combinations;

fn read(path: &str) -> io::Result<Vec<u32>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        let v = line.parse();
        if v.is_err() {
            return Err(io::Error::new(ErrorKind::Other, "Not a number"));
        }
        rv.push(v.unwrap());
    }
    Ok(rv)
}

fn handle_expenses(expenses: &[u32], entries_sum: u32, window: usize) -> Option<Vec<u32>> {
    let combi: Vec<_> = Combinations::new(expenses.to_vec(), window).collect();
    for window in combi {
        let s = window.iter().sum::<u32>();
        if s == entries_sum {
            return Some(window.to_vec());
        }
    }
    None
}

fn mult(v: &[u32]) -> u32 {
    let mut total = 1;
    for n in v {
        total *= n;
    }
    total
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let pair_size = env::args()
        .nth(2).expect("please supply a size")
        .parse().expect("need a pair size");
    let contents = read(&path).expect("no content");

    let v = handle_expenses(&contents, 2020, pair_size);
    match v {
        Some(v) => println!("v={:?} => {}", v, mult(&v)),
        None => println!("Did not find entry adding to 2020"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, window_size, output,
    case(&[1721, 979, 366, 299, 675, 1456], 2, &[299, 1721]),
    case(&[1721, 979, 366, 299, 675, 1456], 3, &[366, 675, 979]),
    )]
    fn test_handle_window(input: &[u32], window_size: usize, output: &[u32]) {
        let rv = handle_expenses(input, 2020, window_size);
        assert!(rv.is_some());
        let mut rv = rv.unwrap().to_vec();
        rv.sort();
        assert_eq!(rv, output.to_vec());
    }

    #[rstest(input, output,
    case(&[299, 1721], 514579),
    case(&[366, 675, 979], 241861950),
    )]
    fn test_mult(input: &[u32], output: u32) {
        assert_eq!(mult(input), output);
    }
}
