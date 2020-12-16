use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

struct Rule {
    field: String,
    ranges: Vec<(usize, usize)>,
}

impl Rule {
    fn from_string(value: &str) -> Rule {
        // println!("Rule from_string: '{}'", value);
        let entries: Vec<&str> = value.split(":").collect();
        let field = entries[0];
        let entries = entries[1].replace(" ", "");
        let entries: Vec<&str> = entries.split("or").collect();

        let mut ranges = Vec::new();
        for values in entries {
            let values: Vec<&str> = values.split("-").collect();
            let v: (usize, usize) = (
                values[0].parse().unwrap(),
                values[1].parse().unwrap(),
            );
            ranges.push(v);
        }

        Rule {
            field: field.to_string(),
            ranges
        }
    }

    fn is_valid(&self, value: usize) -> bool {
        for r in &self.ranges {
            if value >= r.0 && value <= r.1 {
                return true;
            }
        }
        false
    }
}

struct Input {
    rules: Vec<Rule>,
    my_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

impl Input {
    fn read(path: &str) -> io::Result<Input> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut read_rules = true;
        let mut read_nearby_tickets = false;

        let mut rules = Vec::new();
        let mut my_ticket = Vec::new();
        let mut nearby_tickets = Vec::new();

        for line in br.lines() {
            let line = line?;

            if line.is_empty() {
                continue;
            } else if line.starts_with("your ticket") {
                read_rules = false;
                continue;
            } else if line.starts_with("nearby tickets") {
                read_nearby_tickets = true;
                continue;
            }
            // now read what's appropriate

            if read_rules {
                rules.push(Rule::from_string(&line));
            } else {
                let ticket = line.split(",").map(|v| v.parse().unwrap()).collect();
                if read_nearby_tickets {
                    nearby_tickets.push(ticket);
                } else {
                    my_ticket = ticket;
                }
            }

        }
        Ok(Input {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }

    fn ticket_err_rate(&self, ticket: &[usize]) -> usize {
        let mut err_rate = 0;
        for v in ticket {
            let mut valid_rules = false;
            for r in &self.rules {
                if r.is_valid(*v) {
                    valid_rules = true;
                    break
                }
            }
            if !valid_rules {
                err_rate += *v;
            }
        }
        err_rate
    }

    fn compute_err_rate(&self) -> usize {
        self.nearby_tickets.iter().map(|t| self.ticket_err_rate(t)).sum()
    }

    fn discard_invalid_tickets(&mut self) {
        let mut new_tickets = Vec::new();
        let mut invalid_tickets = 0 as usize;

        for t in self.nearby_tickets.clone() {
            if self.ticket_err_rate(&t) == 0 {
                new_tickets.push(t);
            } else {
                invalid_tickets += 1;
            }
        }

        println!("Discarded {}/{} invalid tickets", invalid_tickets, self.nearby_tickets.len());
        self.nearby_tickets = new_tickets;
    }

    fn collapse(guesses: &[HashMap<String, usize>]) -> Vec<String> {
        let mut new_choices:Vec<Vec<String>> = Vec::new();
        // remove impossible solutions
        for g in guesses {
            let max_v = g.values().max().unwrap();
            let new_g: Vec<String> = g.iter()
                .filter(|&(_, v)| v == max_v)
                .map(|(k, _)| k.clone())
                .collect();
            new_choices.push(new_g);
        }
        let mut collapsed = HashSet::new();

        while collapsed.len() < new_choices.len() {
            // choose a value to collapse
            let mut collapse = "".to_string();
            for v in &new_choices {
                if v.len() == 1 {
                    let v = v.first().unwrap();
                    if !collapsed.contains(v) {
                        collapsed.insert(v.clone());
                        collapse = v.clone();
                        break;
                    }
                }
            }
            // iterate new_choices and remove all ref to v
            for v in new_choices.iter_mut() {
                if v.len() > 1 {
                    let to_rem: Vec<usize> = v.iter().enumerate()
                        .filter(|&(_, e)| *e == collapse)
                        .map(|(i, _)| i)
                        .collect();
                    for e in to_rem {
                        v.remove(e);
                    }
                }
            }
        }

        new_choices.iter().map(|v|v.first().unwrap().clone()).collect()
    }

    fn guess_fields(&self) -> Vec<String> {
        let mut guesses :Vec<HashMap<String, usize>> = Vec::new();

        for t in &self.nearby_tickets {
            // println!("considering ticket {:?}", t);
            for (i, v) in t.iter().enumerate() {
                for r in &self.rules {
                    if r.is_valid(*v) {
                        // println!("guesses.len()={} i={}", guesses.len(), i);
                        while guesses.len() <= i {
                            guesses.push(HashMap::new());
                        }
                        if let Some(guess) = guesses.get_mut(i) {
                            let v = guess.get(&r.field).unwrap_or(&0);
                            guess.insert(r.field.clone(), v+1);
                        } else {
                            eprintln!("thought it could not happen!");
                        }
                    }
                }
            }
            // println!("Guesses are now: {:?}", guesses);
        }
        Self::collapse(&guesses)
    }

    fn check_my_ticket(&self, columns: &[String], contains: &str) -> usize {
        let mut rv = 1 as usize;
        println!("Checking my ticket for {}", contains);
        for (i, c) in columns.iter().enumerate() {
            if c.starts_with(contains) {
                let v = &self.my_ticket[i];
                println!("=> {}: {}", c, v);
                rv *= v;
            }
        }
        rv
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let mut contents = Input::read(&path).expect("no content");

    let err_rate = contents.compute_err_rate();
    println!("Q1: err rate: {}", err_rate);

    contents.discard_invalid_tickets();

    let guesses = contents.guess_fields();
    // println!("Cols: {:?}", guesses);

    let rv = contents.check_my_ticket(&guesses, &"departure");
    println!("Q2: {}", rv);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_err_rate,
    case(&"day_16/test_1.txt", 71),
    case(&"day_16/input.txt", 27870),
    )]
    fn test_err_rate(path: &str, exp_err_rate: usize) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(contents.compute_err_rate(), exp_err_rate);
    }

    #[rstest(path, columns,
    case(&"day_16/test_1.txt", vec!("row", "class", "seat")),
    case(&"day_16/input.txt", vec!(
        "price", "train", "duration", "seat", "arrival location", "departure location",
        "arrival track", "zone", "arrival station", "route", "departure date", "arrival platform",
        "row", "departure track", "wagon", "type", "class", "departure platform",
        "departure station", "departure time")),
    )]
    fn test_guess_fields(path: &str, columns: Vec<&str>) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let mut contents = contents.unwrap();
        contents.discard_invalid_tickets();
        assert_eq!(contents.guess_fields(), columns);
    }

    #[rstest(path, columns, exp_check,
    case(&"day_16/test_1.txt", vec!("departure time", "departure zone", "seat"), 7),
    case(&"day_16/test_1.txt", vec!("departure time", "seat", "departure zone"), 7*14),
    case(&"day_16/input.txt", vec!(
        "price", "train", "duration", "seat", "arrival location", "departure location",
        "arrival track", "zone", "arrival station", "route", "departure date", "arrival platform",
        "row", "departure track", "wagon", "type", "class", "departure platform",
        "departure station", "departure time"), 3173135507987),
    )]
    fn test_check_ticket(path: &str, columns: Vec<&str>, exp_check: usize) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();

        let columns: Vec<String> = columns.iter().map(|&f| f.to_string()).collect();
        assert_eq!(contents.check_my_ticket(&columns, &"departure"), exp_check);
    }
}
