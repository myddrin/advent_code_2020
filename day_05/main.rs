use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Seat {
    row: usize,
    seat: usize,  // 0=Leftmost, 7=rightmost
}

fn update_range(mut range: (usize, usize), upper: bool) -> (usize, usize) {
    let half_size = ((range.1 - range.0) + 1) / 2;
    if upper {
        (range.0, range.1 - half_size)
    } else {
        (range.0 + half_size, range.1)
    }
}

impl Seat {
    fn from_string(val: &String) -> Seat {
        let mut row_range = (0, 127);
        let mut seat_range = (0, 7);

        // eprintln!("init ranges row={:?} seat={:?}", row_range, seat_range);
        for c in val.chars() {
            if c == 'F' || c == 'B' {
                row_range = update_range(row_range, c == 'F');
            } else if c == 'R' || c == 'L' {
                seat_range = update_range(seat_range, c == 'L');
            }
            // eprintln!(" {} -> ranges row={:?} seat={:?}", c, row_range, seat_range);
        }

        Seat{row:row_range.0, seat:seat_range.1}
    }

    fn read(path: &str) -> io::Result<Vec<Seat>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for line in br.lines() {
            let line = line?;

            rv.push(Seat::from_string(&line));
        }
        Ok(rv)
    }

    fn id(&self) -> usize {
        self.row * 8 + self.seat
    }
}

fn find_my_seat(seats: &[Seat]) -> Option<Seat> {
    let seat_map :HashSet<usize> = seats.iter().map(|s| s.id()).collect();

    let max_seat = seat_map.iter().max().unwrap();
    println!("Q1: Max id is {}", max_seat);

    for i in (1 as usize..*max_seat) {
        if seat_map.contains(&i) {
            continue;
        }
        // this is an empty seat but to be my seat id+1 and id-1 have to be taken
        if seat_map.contains(&(i - 1)) && seat_map.contains(&(i + 1)) {
            // eprintln!("Found possible seat id={}", i);
            return Some(Seat{
                row: i / 8,
                seat: i % 8,
            })
        }
    }

    None
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Seat::read(&path).expect("no content");

    println!("Loaded {} seats", contents.len());
    let my_seat = if contents.len() > 0 {
        find_my_seat(&contents)
    } else {
        None
    };

    if my_seat.is_some() {
        let my_seat = my_seat.unwrap();
        println!("Q2: My seat is {:?} id={}", my_seat, my_seat.id());
    } else {
        println!("No free seat :(");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(range, upper, exp_range,
    case((0, 127), true, (0, 63)),
    case((0, 63), false, (32, 63)),
    case((32, 63), true, (32, 47)),
    case((32, 47), false, (40, 47)),
    case((40, 47), false, (44, 47)),
    case((44, 47), true, (44, 45)),
    case((44, 45), true, (44, 44)),
    )]
    fn test_update_range(range: (usize, usize), upper: bool, exp_range: (usize, usize)) {
        let range = update_range(range, upper);
        assert_eq!(range, exp_range);
    }

    #[rstest(input, exp_row, exp_seat, exp_id,
    case(&"FBFBBFFRLR", 44, 5, 357),
    case(&"BFFFBBFRRR", 70, 7, 567),
    case(&"FFFBBBFRRR", 14, 7, 119),
    case(&"BBFFBBFRLL", 102, 4, 820),
    )]
    fn test_boarding_pass(input: &str, exp_row: usize, exp_seat: usize, exp_id: usize) {
        let seat = Seat::from_string(&input.to_string());
        println!("Loaded {:?}", seat);
        assert_eq!(seat.row, exp_row);
        assert_eq!(seat.seat, exp_seat);
        assert_eq!(seat.id(), exp_id);
    }
}
