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
        rv.push(line.parse().unwrap());
    }
    Ok(rv)
}

static HANDSHAKE_SUBJECT: usize = 7;
static HANDSHAKE_DIVIDER: usize = 20201227;

fn encrypt(subject: usize, loop_size: usize, divider: usize) -> usize {
    let mut current = 1;
    let mut i = 1;
    while i <= loop_size {
        current = (current * subject) % divider;
        i += 1;
    }
    current
}

fn brute_force(door_key: usize, fob_key: usize) -> (usize, usize) {
    let mut door_loop_size = 0;
    let mut fob_loop_size = 0;
    let mut current_loop = 1;

    let mut value = 1;

    while door_loop_size == 0 || fob_loop_size == 0 {
        if current_loop % 1000 == 0 {
            println!("Trying loop size {} (door_loop_size={}, fob_loop_size={})",
                current_loop,
                door_loop_size,
                fob_loop_size,
            );
        }
        // to be faster we keep the value.
        // let value = encrypt(HANDSHAKE, current_loop, DIVIDER);
        value = (value * HANDSHAKE_SUBJECT) % HANDSHAKE_DIVIDER;

        if value == door_key {
            println!("Door loop: {}", current_loop);
            door_loop_size = current_loop;
        }
        if value == fob_key {
            println!("Fob loop: {}", current_loop);
            fob_loop_size = current_loop;
        }
        current_loop += 1;
    }

    (door_loop_size, fob_loop_size)
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");
    let door_pub_key = contents[0];
    let fob_pub_key = contents[1];

    let start = Instant::now();
    let (door_loop_size, fob_loop_size) = brute_force(door_pub_key, fob_pub_key);
    let duration = start.elapsed();
    println!("Found loop size: door={} fob={} in {:?}",
        door_loop_size,
        fob_loop_size,
        duration,
    );

    let encryption_key = encrypt(door_pub_key, fob_loop_size, HANDSHAKE_DIVIDER);
    println!("Q1: door encryption key: {}", encryption_key);
    let encryption_key = encrypt(fob_pub_key, door_loop_size, HANDSHAKE_DIVIDER);
    println!("Q1: fob encryption key: {}", encryption_key);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(subject, loop_size, exp_key,
    case(HANDSHAKE_SUBJECT, 8, 5764801),
    case(HANDSHAKE_SUBJECT, 11, 17807724),
    case(17807724, 8, 14897079),
    case(5764801, 11, 14897079),
    )]
    fn test_encrypt(subject: usize, loop_size: usize, exp_key: usize) {
        assert_eq!(encrypt(subject, loop_size, HANDSHAKE_DIVIDER), exp_key);
    }

    #[rstest(door_pub_key, fob_pub_key, exp_door_loop_size, exp_fob_loop_size,
    case(17807724, 5764801, 11, 8),
    case(6930903, 19716708, 16190552, 11893237),
    )]
    fn test_brute_force(door_pub_key: usize, fob_pub_key: usize, exp_door_loop_size: usize, exp_fob_loop_size: usize) {
        let (door_loop_size, fob_loop_size) = brute_force(door_pub_key, fob_pub_key);
        assert_eq!(door_loop_size, exp_door_loop_size);
        assert_eq!(fob_loop_size, exp_fob_loop_size);
    }
}
