use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn read(path: &str) -> io::Result<Vec<usize>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        for c in line.split(",") {
            let v: usize = c.parse().unwrap();
            rv.push(v);
        }
    }
    Ok(rv)
}

fn memory_game(first_numbers: &[usize], turns: usize) -> usize {
    let mut memory: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut t = 1;

    // assumes turns > first_numbers.len()
    // we don't store the last number yet
    let mut last_n = 0;
    for n in first_numbers {
        memory.insert(*n, vec!(t));
        // println!("t={} start {}", t, n);
        t += 1;
        last_n = *n;
    }

    while t <= turns {
        let next_n = if memory.contains_key(&last_n) {
            let info = &memory[&last_n];
            if info.len() > 1 {
                // println!("t={} {} was last said at turns {:?}", t, last_n, info);
                info[info.len() - 1] - info[info.len() - 2]
            } else {
                // println!("t={} {} was new!", t, last_n);
                0 as usize
            }
        } else {
            // println!("t={} {} was super new!", t, last_n);
            0 as usize
        };
        let info = memory.get(&next_n);
        if let Some(info) = info {
            // keep only the last 2
            let new_info = vec!(*info.last().unwrap(), t);
            memory.insert(next_n, new_info);
        } else {
            memory.insert(next_n, vec!(t));
        }
        t += 1;
        last_n = next_n;
    }

    last_n
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let last_spoken = memory_game(&contents, 2020);
    println!("Last spoken after 2020 turns: {}", last_spoken);

    let last_spoken = memory_game(&contents, 30000000);
    println!("Last spoken after 30,000,000 turns: {}", last_spoken);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(input, turns, output,
    case(vec!(0, 3, 6), 4, 0),
    case(vec!(0, 3, 6), 9, 4),
    case(vec!(0, 3, 6), 2020, 436),
    case(vec!(1, 3, 2), 2020, 1),
    case(vec!(2, 1, 3), 2020, 10),
    case(vec!(1, 2, 3), 2020, 27),
    case(vec!(2, 3, 1), 2020, 78),
    case(vec!(3, 2, 1), 2020, 438),
    case(vec!(3, 1, 2), 2020, 1836),
    )]
    fn test_memory_game(input: Vec<usize>, turns: usize, output: usize) {
        println!("Running for {} turns", turns);
        assert_eq!(memory_game(&input, turns), output);
    }

    #[rstest(input, turns, output,
    case(vec!(0, 3, 6), 30000000, 175594),
    case(vec!(1, 3, 2), 30000000, 2578),
    )]
    fn test_slow_memory_game(input: Vec<usize>, turns: usize, output: usize) {
        println!("Running for {} turns", turns);
        assert_eq!(memory_game(&input, turns), output);
    }
}
