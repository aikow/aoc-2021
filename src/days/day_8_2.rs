use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer_1() {
    let inputs = parse_input("inputs/day_8.txt");
    println!("Found {} input problems.", inputs.len());
    let mut count = 0;
    for input in inputs {
        for num in input.output {
            let num_segments = num.segments.len() as i32;
            if let Some(_) = check_number(num_segments) {
                count += 1;
            }
        }
    }
    println!("Found {} unique numbers", count);
}

pub fn answer_2() {}

struct Number {
    segments: HashSet<char>,
}

impl Number {
    fn new(wires: &str) -> Number {
        Number {
            segments: HashSet::from_iter(wires.chars()),
        }
    }
}

struct Display {
    top: char,
    tle: char,
    tri: char,
    mid: char,
    bot: char,
    ble: char,
    bri: char,
}

impl Display {}

struct Input {
    signals: Vec<Number>,
    output: Vec<Number>,
}

impl Input {
    fn new(input: String) -> Input {
        let split = input.split("|").collect::<Vec<&str>>();
        match &*split {
            &[wires, digits] => {
                let signals = wires
                    .trim()
                    .split_whitespace()
                    .map(|w| Number::new(w))
                    .collect();
                let output = digits
                    .trim()
                    .split_whitespace()
                    .map(|w| Number::new(w))
                    .collect();
                return Input { signals, output };
            }
            _ => panic!("Failed to split the input"),
        }
    }
}

fn solve_signals(signals: &Vec<Number>) -> Display {
    let one = signals.iter().find(|n| n.segments.len() == 2).unwrap();
    let four = signals.iter().find(|n| n.segments.len() == 4).unwrap();
    let seven = signals.iter().find(|n| n.segments.len() == 3).unwrap();
    let zero_six_nine: Vec<&Number> = signals.iter().filter(|n| n.segments.len() == 6).collect();

    let top: char = *seven.segments.difference(&one.segments).collect::<Vec<char>>().first().unwrap();
    let tle: HashSet<char>;
    let tri: HashSet<char>;
    let mid: HashSet<char>;
    let bot: HashSet<char>;
    let ble: HashSet<char>;
    let bri: HashSet<char>;

}

fn parse_input(filepath: &str) -> Vec<Input> {
    let file = File::open(filepath).expect("Could not open file.");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.expect("Unable to unwrap line read from file.");
            Input::new(line.clone())
        })
        .collect()
}

fn create_knowledge_dict() {
    let mut dict = HashMap::new();
    dict.insert(0, 6);
    dict.insert(1, 2);
    dict.insert(2, 5);
    dict.insert(3, 5);
    dict.insert(4, 4);
    dict.insert(5, 5);
    dict.insert(6, 6);
    dict.insert(7, 3);
    dict.insert(8, 7);
    dict.insert(9, 6);
}

fn check_number(num_segments: i32) -> Option<i32> {
    match num_segments {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}
