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
        rv.push(line.parse().unwrap());
    }
    rv.sort();
    // add my device
    rv.push(rv.iter().max().unwrap() + 3);

    Ok(rv)
}

fn device_jolt(contents: &[usize]) -> &usize {
    contents.iter().max().unwrap()
}

fn device_chain(contents: &[usize]) -> HashMap<usize, usize> {
    let mut diffs = HashMap::new();

    let mut current = 0;
    for adapter in contents {
        let diff = *adapter - current;
        let v = *diffs.get(&diff).unwrap_or(&0);
        diffs.insert(diff, v + 1);
        if diff > 3 {
            eprintln!("Hoho... {} - {} > 3", adapter, current);
        }
        current = *adapter;
    }

    diffs
}

fn find_potential_next(left_contents: &Vec<usize>, current: usize) -> Vec<&usize> {
    left_contents.iter().filter(|&v| *v > current && *v - current <= 3).collect()
}

fn is_valid(contents: &[usize], base: usize, device: usize) -> bool {
    // println!("{:?} can link base={} device={}?", contents, base, device);
    if contents.len() == 0 {
        // eprintln!("  empty");
        return false;
    }
    let usable_content: Vec<&usize> = contents.iter().filter(|&v| *v > base).collect();
    if usable_content.len() == 0 || usable_content[0] - base > 3 {
        // eprintln!("  [0]={:?} - base={} > 3", usable_content.get(0), base);
        return false;
    }
    if device - contents[contents.len() - 1] > 3 {
        // eprintln!("  device={} - [-1]={} > 3", device, contents[contents.len() - 1]);
        return false;
    }

    let mut current = &base;
    for c in usable_content {
        if c - current > 3 {
            // eprintln!("  c={} - current={} > 3", c, current);
            return false;
        }
        current = c;
    }

    true
}

#[derive(Clone,Debug)]
struct Combi {
    combination: Vec<usize>,
    left: Vec<usize>,
}

impl Combi {
    fn find_potential_next(&self) -> Vec<&usize> {
        let current = if self.combination.len() > 0 {
            self.combination.get(self.combination.len() - 1).unwrap()
        } else {
            &0
        };
        self.left.iter().filter(|&v| v > current && v - current <= 3).collect()
    }
}

// That does not work, it accumulates too much for the input.
// I have to think about a recursive way to solve that.
fn find_all_chains(contents: &[usize]) -> usize {
    // assumes contents is sorted
    let mut left = contents.to_vec();
    let device = *left.iter().max().unwrap();
    left.remove(left.len() - 1);  // remove the max entry: the device
    let mut possible = 0;  // the sorted list is valid

    let mut combinations: Vec<Combi> = Vec::new();

    combinations.push(Combi {
        combination: Vec::new(),
        left: left.clone(),
    });

    let mut i: usize = 0;
    println!("== {} init combinations (device={})==", combinations.len(), device);
    while combinations.iter().filter(|&c|c.left.len() > 0).count() > 0 {
        let mut new_combinations: Vec<Combi> = Vec::new();
        for c in combinations {
            i += 1;
            let potential = c.find_potential_next();
            // println!("  found {}", potential.len());
            for v in potential {
                let mut vector = c.combination.clone();
                vector.push(*v);
                if is_valid(&vector, 0, device) {
                    possible += 1;
                }
                let mut new_left: Vec<usize> = c.left.clone()
                    .into_iter()
                    .filter(|&o| o > *v)
                    .collect();
                new_left.sort();
                if new_left.len() > 0 {
                    let vec_max = *vector.iter().max().unwrap();
                    // println!("left is {:?}", left);
                    // println!("new_left is {:?}", new_left);
                    // println!("v={:?}", v);
                    assert!(new_left.len() < left.len());
                    if is_valid(&new_left, vec_max, device) {
                        new_combinations.push(Combi {
                            combination: vector,
                            left: new_left,
                        });
                    }
                }
            }
        }

        // println!("== {} combinations ==", new_combinations.len());
        combinations = new_combinations;
    }

    println!("Found {} combi in {} iter", possible, i);
    possible
}

fn smart_find_all_chains(contents: &[usize]) -> usize {
    // let mut combination_count = vec![0 as usize; contents.len()];
    let mut combination_count = Vec::new();

    println!("{:?}", contents);
    for (idx, a) in contents.iter().enumerate() {
        let start = if idx > 3 {
            idx - 3
        } else {
            0
        };

        let v: usize = contents[start..idx].iter().enumerate()
            .filter(|&(_, b)| *a <= b + 3 && b < a)
            .map(|(j, _)| *combination_count.get(j + start).unwrap_or(&0))
            .sum();
        let v = v + if *a <= 3 {1} else {0};
        // bonus +1 for start connection
        combination_count.push(v);
    }
    combination_count[combination_count.len() - 1]
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    println!("{} adapters", contents.len());
    let device = device_jolt(&contents);
    println!("device is {} jolts", device);

    let diffs = device_chain(&contents);
    println!("Found diffs {:?} => {}",
        diffs,
        diffs.get(&1).unwrap_or(&0) * diffs.get(&3).unwrap_or(&0),
    );

    let possible = smart_find_all_chains(&contents);
    println!("Possible chains: {}", possible);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, arrangements,
    case(&"day_10/test_1.txt", 8),
    case(&"day_10/test_2.txt", 19208),
    )]
    fn test_find(path: &str, arrangements: usize) {
        let contents = read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(find_all_chains(&contents), arrangements);
    }

    #[rstest(path, arrangements,
    case(&"day_10/test_1.txt", 8),
    case(&"day_10/test_2.txt", 19208),
    case(&"day_10/input.txt", 6908379398144),
    )]
    fn test_smart(path: &str, arrangements: usize) {
        let contents = read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(smart_find_all_chains(&contents), arrangements);
    }
}
