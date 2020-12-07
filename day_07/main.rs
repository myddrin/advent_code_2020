use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BagRule {
    colour: String,
    contains: HashMap<String, usize>,
}

impl BagRule {
    fn read(path: &str) -> io::Result<Vec<BagRule>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for line in br.lines() {
            let line = line?;
            rv.push(BagRule::from_string(line));
        }
        Ok(rv)
    }

    fn from_string(value: String) -> BagRule {
        let rules: Vec<&str> = value.split(" contain ").collect();
        let mut contains = HashMap::new();

        for entry in rules[1].split(", ") {
            let rule_parts: Vec<&str> = entry.split_whitespace().collect();
            // I think it's time to learn regex...
            let amount = rule_parts[0]
                .parse().unwrap_or(0);
            if amount == 0 {
                break;  // "no other bags"
            }
            let mut contains_colour = String::new();
            for w in rule_parts.get(1..).unwrap() {
                if w.starts_with("bag") {
                    break;
                }
                contains_colour += " ";
                contains_colour += w;
            }
            contains.insert(contains_colour.trim().to_string(), amount);
        }

        BagRule {
            colour: rules[0].replace(" bags", ""),
            contains,
        }
    }

    fn find_rules(rules: &Vec<BagRule>, colour: &str) -> HashSet<String> {
        let mut can_contain = HashSet::new();
        for rule in rules.iter().filter(|&r| r.contains.contains_key(colour)) {
            can_contain.insert(rule.colour.clone());
        }

        // clone can_contain to allow modification of the original
        // this is recursive... We should stop when we find rules that cannot contain bags.
        for colour in can_contain.clone() {
            for more in Self::find_rules(rules, &colour) {
                can_contain.insert(more);
            }
        }

        can_contain
    }

    fn count_bags(rules: &Vec<BagRule>, colour: &str) -> usize {
        let my_rule = rules.iter()
            .filter(|&r| r.colour == colour)
            .next().unwrap();  // should be only 1

        let mut carry = my_rule.contains.values().sum();
        println!("{} can carry {} from {:?}", colour, carry, my_rule.contains.keys());

        // Then for every bad we can carry inside this one check the ones inside.
        // This is recursive and will stop when we reach a rule that has no bags inside.
        for rule in &my_rule.contains {
            carry += Self::count_bags(rules, &rule.0) * rule.1;
        }

        carry
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = BagRule::read(&path).expect("no content");

    println!("Found {} rules", contents.len());
    let carry_on = BagRule::find_rules(&contents, &"shiny gold");
    println!("Q1: Found {} possibly carry on", carry_on.len());

    let must_carry = BagRule::count_bags(&contents, &"shiny gold");
    println!("Q2: We must carry {} bags inside a shiny gold bag", must_carry);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key.to_string(), $value as usize);
                )+
                m
            }
         };
    );

    #[rstest(input, colour, contains,
    case(
        &"light red bags contain 1 bright white bag, 2 muted yellow bags.",
        &"light red",
        map!{&"bright white" => 1, &"muted yellow" => 2},
    ),
    case(
        &"dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        &"dark orange",
        map!{&"bright white" => 3, &"muted yellow" => 4},
    ),
    case(
        &"faded blue bags contain no other bags.",
        &"faded blue",
        HashMap::new(),
    ),
    )]
    fn test_from_string(input: &str, colour: &str, contains: HashMap<String, usize>) {
        let rule = BagRule::from_string(input.to_string());
        println!("Loaded {:?}", rule);
        assert_eq!(rule.colour, colour);
        assert_eq!(rule.contains, contains);
    }

    #[rstest(path, colour, containers,
    case(&"day_07/test_1.txt", &"shiny gold", 4),
    )]
    fn test_can_contain(path: &str, colour: &str, containers: usize) {
        let rules = BagRule::read(path).unwrap();
        println!("Loaded {} rules", rules.len());
        let found = BagRule::find_rules(&rules, colour);
        println!("Found {:?}", found);
        assert_eq!(found.len(), containers);
    }

    #[rstest(path, colour, must_carry,
    case(&"day_07/test_1.txt", &"shiny gold", 32),
    case(&"day_07/test_2.txt", &"shiny gold", 126),
    )]
    fn test_must_carry(path: &str, colour: &str, must_carry: usize) {
        let rules = BagRule::read(path).unwrap();
        println!("Loaded {} rules", rules.len());
        let carry = BagRule::count_bags(&rules, colour);
        assert_eq!(carry, must_carry);
    }
}
