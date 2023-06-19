use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Ingredient {
    name: String,
    count: usize,
    potential_allergen: HashSet<String>,
}

impl Ingredient {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            count: 0,
            potential_allergen: HashSet::new(),
        }
    }

    fn read(path: &str) -> io::Result<(HashMap<String, Ingredient>)> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv_i: HashMap<String, Ingredient> = HashMap::new();
        // re-using the same object structure to store allergns we know about.
        let mut rv_a: HashMap<String, Ingredient> = HashMap::new();

        for line in br.lines() {
            let line = line?;
            let line = line
                .replace("(", "")
                .replace(")", "")
                .replace(",", "");
            let mut read_ingredients = true;
            let mut ingredients = Vec::new();
            let mut allergns = Vec::new();
            for w in line.split_whitespace() {
                if w == "contains" {
                    read_ingredients = false;
                    continue;
                }
                if read_ingredients {
                    ingredients.push(w.to_string());
                } else {
                    allergns.push(w.to_string());
                }
            }

            for i in ingredients.iter() {
                let i = i.to_string();
                let i = if let Some(i) = rv_i.get_mut(&i) {
                    i
                } else {
                    rv_i.insert(i.clone(), Ingredient::new(&i));
                    rv_i.get_mut(&i).unwrap()
                };
                i.count += 1;
                // count the ingredient occurence, the allergns are handled at the end
            }

            for a in allergns {
                let a = a.to_string();
                if !rv_a.contains_key(&a) {
                    // new allergns are marked on all the ingredients in the current food line
                    println!("Handling new {}", a);
                    let mut alle = Ingredient::new(&a);
                    for i in ingredients.iter() {
                        alle.potential_allergen.insert(i.to_string());
                    }
                    rv_a.insert(a.clone(), alle);
                } else {
                    // known allergns only take the intersection of ingredients
                    println!("Handling known {}", a);
                    let mut alle = rv_a.get_mut(&a).unwrap();
                    let mut new_set = HashSet::new();

                    for i in &alle.potential_allergen {
                        if ingredients.contains(i) {
                            new_set.insert(i.clone());
                        }
                    }

                    alle.potential_allergen = new_set;
                }
            }
        }

        // for all the known allergn, mark them on their ingredients
        for alle in rv_a.values() {
            for i in &alle.potential_allergen {
                let i = rv_i.get_mut(i).unwrap();
                i.potential_allergen.insert(alle.name.clone());
            }
        }

        Ok(rv_i)
    }

    fn search_safe_food(ingredients: &HashMap<String, Ingredient>) -> Vec<&Ingredient> {
        ingredients.values().filter(|&i| i.potential_allergen.len() == 0).collect()
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let ingredients= Ingredient::read(&path).expect("no content");

    println!("Loaded {} ingredients", ingredients.len());
    for i in ingredients.values() {
        println!("{:?}", i);
    }
    // println!("Loaded {} _allergns", allergns.len());
    // for i in allergns.values() {
    //     println!("{:?}", i);
    // }
    let safe = Ingredient::search_safe_food(&ingredients);
    let count: usize = safe.iter().map(|&i|i.count).sum();
    println!("Q1: found {} safe ingredients for a count of {}", safe.len(), count);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_count,
    case("day_21/test_1.txt", 5),
    case("day_21/input.txt", 2485),
    )]
    fn test_search_safe_food(path: &str, exp_count: usize) {
        let ingredients= Ingredient::read(&path).expect("no content");
        let safe = Ingredient::search_safe_food(&ingredients);
        let count: usize = safe.iter().map(|&i|i.count).sum();
        assert_eq!(count, exp_count);
    }
}
