use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer_1() {
    // let syntax_list = parse_input("inputs/test/day_10_test.txt");
    let syntax_list = parse_input("inputs/day_10.txt");
    println!("Found {} expressions to parse", syntax_list.len());
    println!("The score is {}", evaluate_score(&syntax_list));
}

pub fn answer_2() {
    // let syntax_list = parse_input("inputs/test/day_10_test.txt");
    let syntax_list = parse_input("inputs/day_10.txt");
    // println!("Found {} expressions to parse", syntax_list.len());

    let mut scores = Vec::new();

    for expression in &syntax_list {
        if let Err(SyntaxError::UnclosedOpening(mut opened)) = check(expression) {
            // println!("The expression can be closed by {}", opened.iter().join(""));
            let mut expr_score = 0_i128;
            while let Some(c) = opened.pop() {
                expr_score *= 5;
                expr_score += points_2(&c);
            }
            scores.push(expr_score);
        }
    }
    scores.sort();
    let mid = scores.len() / 2;
    let score = scores[mid];

    println!("The score is {}", score);
}

fn parse_input(filepath: &str) -> Vec<Vec<char>> {
    let file = File::open(filepath).expect("Could not open file.");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect()
}

enum SyntaxError {
    /// Expected, actual
    WrongClosing { expected: char, actual: char },

    /// The unexpected parenthesis
    UnexpectedClosing(char),

    /// List of unclosed parenthesis
    UnclosedOpening(Vec<char>),
}

fn check(expression: &Vec<char>) -> Result<(), SyntaxError> {
    let mut stack: Vec<char> = Vec::with_capacity(expression.len() / 2);

    for paren in expression {
        match paren {
            '(' | '[' | '<' | '{' => {
                // Opening parenthesis pushed onto stack
                stack.push(*paren)
            }
            ')' | ']' | '>' | '}' => {
                if let Some(opening) = stack.pop() {
                    if !paren_match(&opening, paren) {
                        // Wrong closing parenthesis
                        return Err(SyntaxError::WrongClosing {
                            expected: opening,
                            actual: *paren,
                        });
                    }
                } else {
                    // Unexpected closing parenthesis
                    return Err(SyntaxError::UnexpectedClosing(*paren));
                }
            }
            _ => panic!("Unexpected parenthesis"),
        }
    }

    if stack.len() > 0 {
        // Unclosed opening parenthesis
        return Err(SyntaxError::UnclosedOpening(
            stack
                .into_iter()
                .map(|p| {
                    return match p {
                        '(' => ')',
                        '[' => ']',
                        '{' => '}',
                        '<' => '>',
                        _ => panic!("Unexpected parenthesis"),
                    };
                })
                .collect_vec(),
        ));
    }

    // Correct syntax
    Ok(())
}

fn paren_match(lhs: &char, rhs: &char) -> bool {
    let expr = String::from_iter(vec![lhs, rhs]);
    return match &expr[..] {
        "()" | "<>" | "[]" | "{}" => true,
        _ => false,
    };
}

fn evaluate_score(expressions: &Vec<Vec<char>>) -> i32 {
    let mut score = 0;

    for expression in expressions {
        if let Err(SyntaxError::WrongClosing { expected, actual }) = check(expression) {
            // println!("Incorrect: expected {}, found {}", expected, actual);
            score += points(&actual);
        }
    }

    return score;
}

fn points(c: &char) -> i32 {
    return match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unexpected symbol"),
    };
}

fn points_2(c: &char) -> i128 {
    return match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unexpected symbol"),
    };
}
