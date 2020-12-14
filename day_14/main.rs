use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::Op::NewMask;
use std::collections::{HashMap, HashSet};

static BITS: usize = 36;

#[derive(Debug,PartialEq,Eq,Clone)]
enum Op {
    NewMask,
    Write,
}

#[derive(Debug,Clone)]
struct Operation {
    op: Op,
    a: usize,  // new mask: X bits, write: address
    b: usize,  // new mask: 1/0 bits, write: value
}

impl Operation {
    fn empty_mask() -> Operation {
        Operation {
            op: NewMask,
            a: (1 << (BITS + 1)) - 1,  // keep all bits
            b: 0,
        }
    }

    fn read(path: &str) -> io::Result<Vec<Operation>> {
        let file = File::open(path)?;
        let br = BufReader::new(file);
        let mut rv = Vec::new();

        for line in br.lines() {
            let line = line?;
            rv.push(Operation::from_string(line));
        }
        Ok(rv)
    }

    fn from_string(line: String) -> Operation {
        use Op::*;
        if line.starts_with("mask = ") {
            let mut or = 0;
            let mut and = 0;
            // mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
            // left = most significant
            let line = line.replace("mask = ", "");
            for (rbit, c) in line.chars().enumerate() {
                let v = 1 << (BITS - rbit -1);
                if c == 'X' {
                    or += v;
                } else if c == '1' {
                    and += v;
                }
            }
            Operation{op: NewMask, a: or, b: and}
        } else {
            let parts: Vec<&str> = line.split(" = ").collect();
            // mem[8] = 11
            Operation {
                op: Write,
                a: parts[0].replace("mem[", "").replace("]", "").parse().unwrap(),
                b: parts[1].parse().unwrap(),
            }
        }
    }

    fn ignore_mask(&self) -> bool {
        use Op::*;
        self.op == NewMask && self.a == (1 << (BITS + 1)) - 1
    }

    fn apply_on_value(&self, value: usize) -> usize {
        if !self.ignore_mask() {
            let v = value & self.a;
            v + self.b
        } else {
            value
        }
    }

    fn apply_on_addr(&self, addr: usize) -> Vec<usize> {
        let base = (addr & (!self.a)) | self.b;
        let mut rv = HashSet::new();
        rv.insert(0);
        for b in 0..BITS+1 {
            if self.a & (1 << b) != 0 {
                for r in rv.clone() {
                    rv.insert(r + (1 << b));
                }
            }
        }
        rv.iter().map(|v| *v | base).collect()
    }

    fn execute_q1(operations: &[Operation]) -> usize {
        use Op::*;
        let mut rv = HashMap::new();
        let mut last_mask = Operation::empty_mask();

        for op in operations {
            // println!("exec {:?} (m={:?})", op, last_mask);
            if op.op == NewMask {
                last_mask = op.clone();
            } else {
                let v = last_mask.apply_on_value(op.b);
                rv.insert(op.a, v);
            }
        }

        rv.values().sum()
    }

    fn execute_q2(operations: &[Operation]) -> usize {
        use Op::*;
        let mut rv = HashMap::new();
        let mut last_mask = Operation::empty_mask();

        for op in operations {
            println!("exec {:?} (m={:?})", op, last_mask);
            if op.op == NewMask {
                last_mask = op.clone();
            } else {
                let addrs = last_mask.apply_on_addr(op.a);
                // println!("  write {} to {} addrs", op.b, addrs.len());
                for a in addrs {
                    rv.insert(a, op.b);
                }
            }
        }

        rv.values().sum()
    }
}

fn main() {
    // Do NOT run test_1, q2 will consume a lot of memory as most masks have a lot of Xs!
    let path = env::args().nth(1).expect("please supply a path");
    let contents = Operation::read(&path).expect("no content");

    println!("Loaded {} operations", contents.len());
    let mem_sum = Operation::execute_q1(&contents);
    println!("Q1: Mem sum {}", mem_sum);

    let mem_sum = Operation::execute_q2(&contents);
    println!("Q2: Mem sum {}", mem_sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest()]
    fn test_empty_mask() {
        assert!(Operation::empty_mask().ignore_mask());
    }

    #[rstest()]
    fn test_from_string() {
        let op = Operation::from_string("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
        assert_eq!(op.b, 64);
    }

    #[rstest()]
    fn test_apply_on_addr() {
        let op = Operation::from_string("mask = 000000000000000000000000000000X1001X".to_string());
        assert_eq!(op.b, 18);
        assert_eq!(op.a, 33);
        assert_eq!(op.apply_on_addr(42), vec!(26, 27, 58, 59))
    }

    #[rstest(path, exp_mem,
    case(&"day_14/test_1.txt", 165),
    )]
    fn test_execute_q1(path: &str, exp_mem: usize) {
        let contents = Operation::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(Operation::execute_q1(&contents), exp_mem);
    }

    #[rstest(path, exp_mem,
    case(&"day_14/test_2.txt", 208),
    )]
    fn test_execute_q2(path: &str, exp_mem: usize) {
        let contents = Operation::read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(Operation::execute_q2(&contents), exp_mem);
    }
}
