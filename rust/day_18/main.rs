use std::{io, env};
use std::fs::File;
use std::io::{BufReader, BufRead};

fn to_expr(line: String) -> Vec<String> {
    line
        .replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|e| e.to_string())
        .collect()
}

fn read(path: &str) -> io::Result<Vec<Vec<String>>> {
    let file = File::open(path)?;
    let br = BufReader::new(file);
    let mut rv = Vec::new();

    for line in br.lines() {
        let line = line?;
        rv.push(to_expr(line));
    }
    Ok(rv)
}

// TODO
fn prioritise_addition(expr: &[String]) -> Vec<String> {
    let mut rv = Vec::new();
    let mut push = 0;

    for (i, b) in expr.iter().enumerate() {
        rv.push(b.clone());
        if b == "+" {
            rv.insert(i - 1, "(".to_string());
            push += 1;
        }
    }

    rv
}

fn apply(op: &str, a: i64, b: i64) -> i64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => b  // no operation, just store the value
    }
}

// no precedence
fn evaluate(expr: &[String]) -> i64 {
    let mut prev_block = 0;
    let mut op = "";
    let mut open = 0 as usize;
    let mut acc = Vec::new();

    // println!("store {}", prev_block);
    for b in expr.iter() {
        // println!("{} prev={} op={} open={} acc={:?}", b, prev_block, op, open, acc);
        if b == "(" {
            if open > 0 {
                acc.push(b.clone());
            }
            open += 1;
            continue;
        } else if b == ")" {
            open -= 1;

            if open == 0 {
                // we finished accumulating
                let v = evaluate(&acc);
                // println!("-> {} prev={} op={}", v, prev_block, op);
                if op.is_empty() {
                    prev_block = v;
                } else {
                    prev_block = apply(op, prev_block, v);
                    op = "";
                }
                acc.clear();
            } else {
                acc.push(b.clone());
            }
            continue;
        }

        if open > 0 {
            // we're in a parenthesis block, accumulate and compute on close
            acc.push(b.clone());
            continue;
        }

        let v = b.parse::<i64>();
        match v {
            Ok(v) => {
                prev_block = apply(op, prev_block, v);
                op = "";
            },
            _ => op = b
        }
    }

    println!("=> {} = {}", expr.join(" "), prev_block);
    prev_block
}

fn eval_sum(exprs: &[Vec<String>]) -> i64 {
    exprs.iter().map(|e| evaluate(&e)).sum()
}

fn main() {
    let path = env::args().nth(1).expect("please supply a path");
    let contents = read(&path).expect("no content");

    let sum = eval_sum(&contents);
    println!("Q1: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(expr, exp,
    case("1 + 2 * 3 + 4 * 5 + 6", 71),
    case("2 * 3 + (4 * 5)", 26),
    case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
    case("1 + (2 * 3) + (4 * (5 + 6))", 51),
    case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
    case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
    )]
    fn test_evaluate(expr: &str, exp: i64) {
        let expr: Vec<String> = to_expr(expr.to_string());
        assert_eq!(evaluate(&expr), exp);
    }

    #[rstest(path, exp,
    case("day_18/test_1.txt", 71 + 26 + 437 + 51 + 12240 + 13632),
    case("day_18/input.txt", 9535936849815),
    )]
    fn test_eval_sum(path: &str, exp: i64) {
        let contents = read(&path);
        assert!(contents.is_ok());
        let contents = contents.unwrap();
        assert_eq!(eval_sum(&contents), exp);
    }
}
