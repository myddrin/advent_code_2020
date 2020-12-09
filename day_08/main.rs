use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug,PartialEq,Eq,Clone)]
enum Op {
    Nop,
    Jmp,
    Acc,
}


impl Op {
    fn from_string(value: &str) -> Op {
        use Op::*;
        match value {
            "nop" => Nop,
            "jmp" => Jmp,
            "acc" => Acc,
            &_ => Nop,  // actually a bug
        }
    }
}

#[derive(Debug,Clone)]
struct Instruction {
    op: Op,
    value: i32,
}

impl Instruction {
    fn from_string(line: String) -> Option<Instruction> {
        let entries: Vec<&str> = line.split_whitespace().collect();
        let op = entries.get(0)?;
        let value = entries.get(1)?.parse();
        if value.is_err() {
            return None;
        }
        Some(Instruction {
            op: Op::from_string(op),
            value: value.unwrap(),
            // accumulator_when_visited: None,
        })
    }

    fn read(path: &str) -> io::Result<Vec<Instruction>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for line in br.lines() {
            let line = line?;
            if let Some(op) = Self::from_string(line) {
                rv.push(op);
            } else {
                return Err(io::Error::new(io::ErrorKind::Other, "bad line"));
            }
        }
        Ok(rv)
    }

    fn execute(&self) -> (i32, i32) {
        match self.op {
            Op::Nop => (0, 1),
            Op::Jmp => (0, self.value),
            Op::Acc => (self.value, 1),
        }
    }

    fn to_program(code: &[Instruction]) -> Vec<(&Instruction, Option<i32>)> {
        code.iter().map(|v| (v, None)).collect()
    }

    fn fix_and_run(code: &[Instruction], change: usize) -> Option<i32> {
        let mut code = code.clone().to_vec();
        println!("[Trying to fix {} to nop]", change);
        code[change].op = if code[change].op == Op::Jmp {
            Op::Nop
        } else {
            Op::Jmp
        };
        let (last_acc, _last_cp, found_jmp) = Self::run(&code);
        if found_jmp.is_none() {
            Some(last_acc)
        } else {
            None
        }
    }

    fn run(code: &[Instruction]) -> (i32, i32, Option<Vec<usize>>) {
        let mut code = Self::to_program(code);
        let mut accumulator = 0;
        let mut code_pointer: i32 = 0;
        let mut found_jump: Vec<usize> = Vec::new();

        while code_pointer < code.len() as i32 {
            let cp = code_pointer as usize;
            if code[cp].0.op != Op::Acc {
                found_jump.push(cp);
            }
            let (mod_acc, mod_cp) = code[cp].0.execute();

            accumulator += mod_acc;
            code[cp].1 = Some(accumulator);
            code_pointer += mod_cp;


            // do it before finishing so we can return the offending CP
            let next = code.get(code_pointer as usize);
            if let Some(next) = next {
                if next.1.is_some() {
                    eprintln!("{:?} loop!, found {} jumps", code[cp], found_jump.len());
                    return (accumulator, cp as i32, Some(found_jump));  // last accumulator value
                }
            } else if code_pointer < 0 {
                eprintln!("{:?} neg cp!, found {} jumps", code[cp], found_jump.len());
                return (accumulator, cp as i32, Some(found_jump));  // last accumulator value
            }
        }

        eprintln!("Finished");
        (accumulator, code_pointer, None)
    }
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Instruction::read(&path).expect("no content");

    println!("Loaded {} lines of code", contents.len());
    // eprintln!("{:?}", contents);

    let (last_acc, last_cp, jmps) = Instruction::run(&contents);
    println!("Last accumulator: {} (cp={})", last_acc, last_cp);

    if let Some(jmps) = jmps {
        for jmp_offset in jmps {
            let rv = Instruction::fix_and_run(&contents, jmp_offset);
            if let Some(rv) = rv {
                println!("Fix the progam at {} and the accumulator is {}", jmp_offset, rv);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(path, exp_last_acc, exp_last_cp,
    case(&"day_08/test_1.txt", 5, 4),
    )]
    fn test_find_loop(path: &str, exp_last_acc: i32, exp_last_cp: i32) {
        let contents = Instruction::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        let (last_acc, last_cp, _jmps) = Instruction::run(&contents);
        assert_eq!(last_acc, exp_last_acc);
        assert_eq!(last_cp, exp_last_cp);
    }

    #[rstest(path, fix_at, exp_last_acc,
    case(&"day_08/test_1.txt", 7, Some(8)),
    case(&"day_08/test_1.txt", 2, None),
    )]
    fn test_fix_and_run(path: &str, fix_at: usize, exp_last_acc: Option<i32>) {
        let contents = Instruction::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(Instruction::fix_and_run(&contents, fix_at), exp_last_acc);
    }
}
