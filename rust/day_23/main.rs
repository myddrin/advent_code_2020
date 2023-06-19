use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

fn read(path: &str) -> io::Result<Vec<usize>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        for c in line.chars() {
            rv.push(c.to_string().parse().unwrap());
        }
    }
    Ok(rv)
}

#[derive(Debug)]
struct Game {
    cups: Vec<usize>,
    current: usize,
}

impl Game {
    fn new(cups: &[usize]) -> Self {
        Self {
            cups: cups.to_vec(),
            current: cups[0],
        }
    }

    fn new_translated(cups: &[usize], until: usize) -> Self {
        let mut rv = Self::new(cups);
        let mut i = rv.cups.iter().max().unwrap() + 1;
        while i < until {
            rv.cups.push(i);
            i += 1;
        }
        rv
    }

    fn current_idx(&self) -> usize {
        self.cups.iter().enumerate()
            .filter(|&(_, c)| *c == self.current)
            .map(|(i, _)| i)
            .next().unwrap()
    }

    // return the selected cups
    fn pickup(&mut self, n: usize) -> Vec<usize> {
        let mut rv = Vec::new();
        while rv.len() < n {
            let pick_from = (self.current_idx() + 1) % self.cups.len();
            rv.push(self.cups[pick_from]);
            self.cups.remove(pick_from);
        }
        rv
    }

    // returns the position of the destination
    fn destination(&self) -> usize {
        let mut dest = self.current - 1;
        let min = self.cups.iter().min().unwrap();
        let max = self.cups.iter().max().unwrap();

        loop {
            if let Some(rv) = self.cups.iter()
                .enumerate()
                .filter(|&(_, c)| *c == dest)
                .map(|(i, _)| i)
                .next() {
                return rv;  // position in cups
            }
            // we don't have the current label-1, so we keep reducing until we find it!
            if dest > *min {
                dest -= 1;
            } else {
                dest = *max;
            }
        }
    }

    fn play(&mut self) {
        // println!("Current: {} ({})", self.current, self.current_idx());
        let mut pickup = self.pickup(3);
        let dest = self.destination();
        // println!("Selected {:?} and insert them after {} ({})",
        //     pickup, self.cups[dest], dest,
        // );
        pickup.reverse();
        for c in pickup {
            self.cups.insert(dest + 1, c);
        }
        self.current = self.cups[(self.current_idx() + 1) % self.cups.len()];
    }

    fn play_for(&mut self, turns: usize) -> String {
        let mut t = 1;
        println!("Running for {} turns...", turns);
        while t <= turns {
            // if self.cups.len() < 10 {
            //     println!("t={} {:?}", t, self);
            // }
            if t % 1000 == 0 {
                println!("t={} current={} ({})", t, self.current, self.current_idx());
            }
            self.play();
            t += 1;
        }

        // find index of cup label 1
        let mut idx = self.cups.iter().enumerate()
            .filter(|&(_, c)| *c == 1)
            .map(|(i, _)| i)
            .next().unwrap();
        let mut rv = Vec::new();
        // we return the cups order ignore cup 1
        while rv.len() < self.cups.len() - 1 {
            idx = (idx + 1) % self.cups.len();
            rv.push(self.cups[idx].to_string());
        }
        rv.join("")
    }

    fn find_after_one(&self) -> (usize, usize) {
        let idx = self.cups.iter().enumerate()
            .filter(|&(_, c)| *c == 1)
            .map(|(i, _)| i)
            .next().unwrap();
        (
            (idx + 1) % self.cups.len(),
            (idx + 2) % self.cups.len(),
        )
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let mut game = Game::new(&contents);
    let rv = game.play_for(100);
    println!("Q1: after 100 turns: {}", rv);

    // let mut game = Game::new_translated(&contents, 1000000);
    // let start = Instant::now();
    // game.play_for(10000000);
    // let duration = start.elapsed();
    // let after_one = game.find_after_one();
    // println!("Q2: after one are {} and {} is {}",
    //     after_one.0,
    //     after_one.1,
    //     after_one.0 * after_one.1,
    // );
    // println!("Done in {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, turns, exp_cups, exp_current,
    case("day_23/test_1.txt", 1, vec!(3, 2, 8, 9, 1, 5, 4, 6, 7), 2),
    // example rotates to keep "current" at the same index and offers 9  2  5  8  4 (1) 3  6  7
    case("day_23/test_1.txt", 5, vec!(4, 1, 3, 6, 7, 9, 2, 5, 8), 1),
    case("day_23/test_1.txt", 10, vec!(5, 8, 3, 7, 4, 1, 9, 2, 6), 8),
    )]
    fn test_play_for(path: &str, turns: usize, exp_cups: Vec<usize>, exp_current: usize) {
        let contents = read(&path).expect("no content");
        let mut game = Game::new(&contents);
        game.play_for(turns);
        println!("{:?}", game);
        assert_eq!(game.cups, exp_cups);
        assert_eq!(game.current, exp_current);
    }

    #[rstest(path, exp_rv,
    case("day_23/test_1.txt", "67384529"),
    case("day_23/input.txt", "82635947"),
    )]
    fn test_play_for_100(path: &str, exp_rv: &str) {
        let contents = read(&path).expect("no content");
        let mut game = Game::new(&contents);
        assert_eq!(game.play_for(100), exp_rv.to_string());
    }

    // #[rstest(path, exp_after_one,
    // case("day_23/test_1.txt", (934001, 159792)),
    // )]
    // fn test_after_one(path: &str, exp_after_one: (usize, usize)) {
    //     let contents = read(&path).expect("no content");
    //     let mut game = Game::new_translated(&contents, 1000000);
    //     game.play_for(10000000);
    //     assert_eq!(game.find_after_one(), exp_after_one);
    // }
}
