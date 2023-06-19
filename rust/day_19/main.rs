use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone,Debug)]
struct Rule {
    name: usize,
    value: String,
    ref_a: Vec<usize>,
    ref_b: Vec<usize>,
}

impl Rule {
    fn from_string(line: String) -> Rule {
        let entries: Vec<&str> = line.split(": ").collect();
        let idx = entries[0].parse().unwrap();
        let rule = if entries[1].starts_with("\"") {
            Rule {
                name: idx,
                value: entries[1].replace("\"", "").to_string(),
                ref_a: Vec::new(),
                ref_b: Vec::new(),
            }
        } else {
            let options: Vec<&str> = entries[1].split(" | ").collect();
            let ref_a = options[0]
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect();
            let ref_b = if let Some(v) = options.get(1) {
                v.split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect()
            } else {
                Vec::new()
            };
            Rule {
                name: idx,
                value: String::new(),
                ref_a,
                ref_b,
            }
        };

        rule
    }

    fn to_regex(&self, rules: &HashMap<usize, Rule>) -> String {
        if !self.value.is_empty() {
            self.value.clone()
        } else {
            let mut rv = Vec::new();

            if !self.ref_a.is_empty() {
                let mut v = String::new();
                for b in &self.ref_a {
                    v += &rules[b].to_regex(rules);
                }
                rv.push(v);
            }
            if !self.ref_b.is_empty() {
                let mut v = String::new();
                for b in &self.ref_b {
                    v += &rules[b].to_regex(rules);
                }
                rv.push(v);
            }

            let rv = rv.join("|");
            "(".to_string() + &rv + ")"
        }
    }
}

struct Input {
    rules: HashMap<usize, Rule>,
    entries: Vec<String>,
}

impl Input {
    // we could return a vector instead of a hashmap
    fn read(path: &str) -> io::Result<Input> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rules = HashMap::new();
        let mut entries = Vec::new();
        let mut read_rules = true;

        for line in br.lines() {
            let line = line?;
            if line.is_empty() {
                read_rules = false;
                continue
            }
            if read_rules {
                let rule = Rule::from_string(line);
                rules.insert(rule.name.clone(), rule);
            } else {
                entries.push(line);
            }
        }
        Ok(Input { rules, entries })
    }

    fn to_regex(&self, rule: usize) -> Regex {
        let r = "^".to_string() + &self.rules[&rule].to_regex(&self.rules) + "$";
        Regex::new(&r).unwrap()
    }

    fn matches_regex(&self, value: &str, rule: usize) -> bool {
        let re = self.to_regex(rule);
        println!("built {:?} vs {}", re, value);
        re.is_match(value)
    }

    fn count_matches(&self, rule: usize) -> usize {
        let re = self.to_regex(rule);
        self.entries.iter()
            .filter(|&v|re.is_match(v))
            .count()
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Input::read(&path).expect("no content");
    println!("Loaded {} rules and {} entries", contents.rules.len(), contents.entries.len());

    let matches_0 = contents.count_matches(0);
    println!("Q1: {} entries match rule 0", matches_0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, check, exp,
    case("day_19/test_1.txt", "ababbb", true),
    case("day_19/test_1.txt", "abbbab", true),
    case("day_19/test_1.txt", "bababa", false),
    case("day_19/test_1.txt", "aaabbb", false),
    case("day_19/test_1.txt", "aaaabbb", false),
    //
    case("day_19/input.txt", "bbabbabaaaaabaaabbaabbab", true),
    )]
    fn test_matches_regex(path: &str, check: &str, exp: bool) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(contents.matches_regex(check, 0), exp);
    }

    #[rstest(path, rule, exp,
    case("day_19/test_1.txt", 4, "^a$"),
    case("day_19/test_1.txt", 3, "^(ab|ba)$"),
    case("day_19/test_1.txt", 1, "^((aa|bb)(ab|ba)|(ab|ba)(aa|bb))$"),
    case("day_19/test_1.txt", 0, "^(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)$"),
    )]
    fn test_regex(path: &str, rule: usize, exp: &str) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(format!("{}", contents.to_regex(rule)), exp);
    }

    #[rstest(path, exp_match,
    case("day_19/test_1.txt", 2),
    case("day_19/test_2.txt", 2),  // same than test_1 but rules are not in order
    case("day_19/input.txt", 279),
    )]
    fn test_count_matches(path: &str, exp_match: usize) {
        let contents = Input::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(contents.count_matches(0), exp_match);
    }
}
