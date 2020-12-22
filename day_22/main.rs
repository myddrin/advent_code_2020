use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;

#[derive(Clone)]
struct Player {
    name: String,
    cards: Vec<usize>,
}

impl Player {
    fn new() -> Self {
        Self {
            name: String::new(),
            cards: Vec::new(),
        }
    }

    fn read(path: &str) -> io::Result<(Player, Player)> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut p1 = Player::new();
        let mut p2 = Player::new();
        let mut read_p1 = true;

        for line in br.lines() {
            let line = line?;
            if line.is_empty() {
                read_p1 = false;
                continue;
            }
            if let Ok(v) = line.parse::<usize>() {
                if read_p1 {
                    p1.cards.push(v);
                } else {
                    p2.cards.push(v);
                }
            } else {
                let name = line.replace(":", "");
                if read_p1 {
                    p1.name = name;
                } else {
                    p2.name = name;
                }
            }
        }
        Ok((p1, p2))
    }

    fn draw(&mut self) -> usize {
        let c: Vec<usize> = self.cards.drain(..1).collect();
        *c.first().unwrap()
    }

    fn recurse_clone(&self, cards: usize) -> Player {
        Player {
            name: self.name.clone(),
            cards: self.cards.clone()[..cards].to_vec(),  // not sure the 1st clone is needed
        }
    }

    // if game is not None we are playing recursive combat
    fn play_combat_round(&mut self, other: &mut Self, game: Option<usize>) -> (bool, Vec<usize>) {
        let p1_card = self.draw();
        let p2_card = other.draw();

        if let Some(game) = game {
            // we're playing a recursive game, check if we should start a new game.
            if self.cards.len() >= p1_card && other.cards.len() >= p2_card {
                let mut p1 = self.recurse_clone(p1_card);
                let mut p2 = other.recurse_clone(p2_card);
                let w = p1.play_recursive_combat(&mut p2, game + 1);
                if w {
                    println!("{} wins the recursive round. Back to game {}", self.name, game);
                    return (w, vec!(p1_card, p2_card));
                } else {
                    println!("{} wins the recursive round. Back to game {}", other.name, game);
                    return (w, vec!(p2_card, p1_card));
                }
            }
            // not enough cards to recurse, let's play a regular round.
        }

        // println!("p1 draws {}, p2 draws {}", p1_card, p2_card);
        if p1_card > p2_card {
            (true, vec!(p1_card, p2_card))
        } else {
            (false, vec!(p2_card, p1_card))
        }
    }

    fn play_combat(&mut self, other: &mut Self) -> bool {
        let mut t: usize = 0;
        println!("{} is playing with {}", self.name, other.name);

        while !self.cards.is_empty() && !other.cards.is_empty() {
            println!("-- Round {} --", t);
            let (w, cards) = self.play_combat_round(other, None);
            if w {
                for c in cards {
                    self.cards.push(c);
                }
            } else {
                for c in cards {
                    other.cards.push(c);
                }
            }
            t += 1;
        }

        !self.cards.is_empty()
    }

    fn play_recursive_combat(&mut self, other: &mut Self, number: usize) -> bool {
        let mut t: usize = 0;
        if number == 0 {
            println!("{} is playing with {}", self.name, other.name);
        } else {
            println!("Recursive game {}", number);
        }
        let mut history = HashSet::new();

        while !self.cards.is_empty() && !other.cards.is_empty() {
            if number == 0 {
                println!("-- Round {} (Game {}) --", t, number);
            }

            let v = (self.cards.clone(), other.cards.clone());
            if history.contains(&v) {
                println!("We hit the same cards configuration! {} wins!", self.name);
                return true;  // p1 wins if we hit the same configuration
            } else {
                history.insert(v);
            }

            let (w, cards) = self.play_combat_round(other, Some(number));
            if w {
                for c in cards {
                    self.cards.push(c);
                }
            } else {
                for c in cards {
                    other.cards.push(c);
                }
            }

            t += 1;
        }

        !self.cards.is_empty()
    }

    fn score(&self) -> usize {
        self.cards.iter().rev()
            .enumerate()
            .map(|(i, v)| (i + 1 ) * v)
            .sum()
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Player::read(&path).expect("no content");

    println!("Let's play combat!");
    let mut p1 = contents.0.clone();
    let mut p2 = contents.1.clone();

    if p1.play_combat(&mut p2) {
        println!("{} wins with {} points", p1.name, p1.score());
    } else {
        println!("{} wins with {} points", p2.name, p2.score());
    }

    println!("Let's play recursive combat!");
    let mut p1 = contents.0.clone();
    let mut p2 = contents.1.clone();
    if p1.play_recursive_combat(&mut p2, 0) {
        println!("{} wins with {} points", p1.name, p1.score());
    } else {
        println!("{} wins with {} points", p2.name, p2.score());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_winner, exp_score,
    case("day_22/test_1.txt", false, 306),
    case("day_22/input.txt", true, 33559),
    )]
    fn test_play_combat(path: &str, exp_winner: bool, exp_score: usize) {
        let (mut p1, mut p2) = Player::read(path).unwrap();

        let w = p1.play_combat(&mut p2);
        assert_eq!(w, exp_winner);
        if w {
            assert_eq!(p1.score(), exp_score);
        } else {
            assert_eq!(p2.score(), exp_score);
        }
    }

    #[rstest(path, exp_winner, exp_score,
    case("day_22/test_1.txt", false, 291),
    case("day_22/input.txt", true, 32789),
    )]
    fn test_play_recursive_combat(path: &str, exp_winner: bool, exp_score: usize) {
        let (mut p1, mut p2) = Player::read(path).unwrap();

        let w = p1.play_recursive_combat(&mut p2, 0);
        assert_eq!(w, exp_winner);
        if w {
            assert_eq!(p1.score(), exp_score);
        } else {
            assert_eq!(p2.score(), exp_score);
        }
    }

    #[rstest()]
    fn test_no_loop_forever() {
        let (mut p1, mut p2) = Player::read("day_22/test_2.txt").unwrap();
        p1.play_recursive_combat(&mut p2, 0);
    }
}
