use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
struct Prediction {
    when: usize,
    timetable: Vec<Option<usize>>
}

impl Prediction {
    fn read(path: &str) -> io::Result<Prediction> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut when = 0;
        let mut timetable = Vec::new();

        for (i, line) in br.lines().enumerate() {
            let line = line?;

            if i == 0 {
                when = line.parse().unwrap();
            } else {
                timetable = line.split(",")
                    .map(|c| if let Ok(v) = c.parse() { Some(v) } else {None})
                    .collect();
            }
        }
        Ok(Prediction { when, timetable })
    }

    fn build_wait(&self) -> HashMap<usize, usize> {
        let mut rv = HashMap::new();
        for bus in &self.timetable {
            if let Some(bus) = bus {
                rv.insert(bus.clone(), (self.when / bus + 1) * bus);
            }
        }
        rv
    }

    // return: smallest bus, wait for that bus
    fn earliest_bus(&self) -> (usize, usize) {
        let wait = self.build_wait();
        println!("built wait: {:?}", wait);
        // that didn't work so well, it used the key
        // let smallest = wait.iter().min_by(|&(_, v)| v).unwrap();
        // I'm lazy for now
        let smallest_wait = wait.values().min().unwrap();
        let smallest_bus: Vec<&usize> = wait.iter()
            .filter(|&(_, v)| v == smallest_wait)
            .map(|(b, _)| b)
            .collect();
        (*smallest_bus[0], (smallest_wait - self.when))
    }

    fn build_sync_table(&self) -> Vec<usize> {
        // x => 1 because the bus can go whenever.
        self.timetable.iter().map(|v| v.unwrap_or(1)).collect()
    }

    fn earliest_synchronous(&self) -> usize {
        let contents = self.build_sync_table();
        let mut rv = vec!();
        let mut i = 0;
        let mut t = contents[0];
        let mut inc = contents[0];

        while rv.len() < contents.len() - 1 {
            let idx = rv.len() + 1;
            let target = contents[idx];
            if (t + 1) % target == 0 {
                println!("i={} rv={:?} + {} for {}", i, rv, t + 1, contents[idx]);
                rv.push(t);
                inc *= contents[idx];
                t += 1;
            } else {
                t += inc;
            }
            i += 1;
        }
        println!("Found in {} iterations", i);
        t - rv.len()
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Prediction::read(&path).expect("no content");

    let (bus, wait) = contents.earliest_bus();
    println!("Q1 need to wait {} min for bus {}: {}",
        wait,
        bus,
        wait * bus,
    );

    let earliest = contents.earliest_synchronous();
    println!("Q2: {}", earliest);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_route,
    case(&"day_13/test_1.txt", (59, 5)),
    )]
    fn test_earliest_bus(path: &str, exp_route: (usize, usize)) {
        let contents = Prediction::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        println!("Loaded {:?}", contents);
        assert_eq!(contents.earliest_bus(), exp_route);
    }

    #[rstest(data, exp_ts,
    // pre-casted the None to 1
    case(vec!(7, 13, 1, 1, 59, 1, 31, 19), 1068781),
    case(vec!(17,1,13,19), 3417),
    case(vec!(67,7,59,61), 754018),
    case(vec!(67,1,7,59,61), 779210),
    case(vec!(67,7,1,59,61), 1261476),
    case(vec!(1789,37,47,1889), 1202161486),
    )]
    fn test_brute_q2(data: Vec<usize>, exp_ts: usize) {
        let contents = Prediction {
            when: 0,  // ignored
            timetable: data.iter().map(|v| Some(*v)).collect(),
        };
        assert_eq!(contents.earliest_synchronous(), exp_ts);
    }
}
