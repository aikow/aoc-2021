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

pub fn answer_2() {
    let inputs = parse_input("inputs/day_8.txt");
    println!("Found {} input problems.", inputs.len());
    let mut count = 0;
    for input in inputs {
        let display = solve_signals(&input.signals);
        println!("{:?}", display);
        count += input.output.iter().rev().enumerate().map(|(i, n)| (10_i32.pow(i as u32)) * display.convert_to_number(n)).sum::<i32>();
    }
    println!("Total of all outputs is {}", count);
}



#[derive(Eq, PartialEq, Debug)]
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

#[derive(Debug)]
struct Display<T> {
    top: T,
    tle: T,
    tri: T,
    mid: T,
    ble: T,
    bri: T,
    bot: T,
}

impl Display<char> {
    fn convert_to_number(&self, number: &Number) -> i32 {
        let zero:HashSet<char> = HashSet::from_iter([self.top, self.tle, self.tri, self.ble, self.bri, self.bot]);
        let one:HashSet<char> = HashSet::from_iter([self.tri, self.bri]);
        let two:HashSet<char> = HashSet::from_iter([self.top, self.tri, self.mid, self.ble, self.bot]);
        let three:HashSet<char> = HashSet::from_iter([self.top, self.tri, self.mid, self.bri, self.bot]);
        let four:HashSet<char> = HashSet::from_iter([self.tle, self.tri, self.mid, self.bri]);
        let five:HashSet<char> = HashSet::from_iter([self.tle, self.tri, self.mid, self.bri]);
        let six:HashSet<char> = HashSet::from_iter([self.top, self.tle, self.mid, self.ble, self.bri, self.bot]);
        let seven:HashSet<char> = HashSet::from_iter([self.top, self.tri, self.bri]);
        let eight:HashSet<char> = HashSet::from_iter([self.top, self.tle, self.tri, self.mid, self.ble, self.bri, self.bot]);
        let nine:HashSet<char> = HashSet::from_iter([self.top, self.tle, self.tri, self.mid, self.bri, self.bot]);
        match number.segments.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            5 => {
                return if two.is_subset(&number.segments) {
                    2
                } else if three.is_subset(&number.segments) {
                    3
                } else {
                    5
                }
            },
            6 => {
                return if zero.is_subset(&number.segments) {
                    0
                } else if six.is_subset(&number.segments) {
                    6
                } else {
                    9
                }
            }
            _ => panic!("Invalid number"),
        }
    }
}

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

fn solve_signals(signals: &Vec<Number>) -> Display<char> {
    let one = signals.iter().find(|n| n.segments.len() == 2).unwrap();
    let four = signals.iter().find(|n| n.segments.len() == 4).unwrap();
    let seven = signals.iter().find(|n| n.segments.len() == 3).unwrap();
    let eight = signals.iter().find(|n| n.segments.len() == 7).unwrap();
    let mut zero_six_nine: Vec<&Number> = signals.iter().filter(|n| n.segments.len() == 6).collect();

    // Fill out the top bar by subtracting the one from the seven
    let top: char = **seven
        .segments
        .difference(&one.segments)
        .collect::<Vec<&char>>()
        .first()
        .unwrap();


    // We know the nine has to share 5 parts with the four and the top part, and
    let nine_parts = four.segments.union(&seven.segments).map(|c| *c).collect::<HashSet<char>>();
    assert_eq!(5, nine_parts.len());
    let nine = *zero_six_nine.iter().find(|&n| nine_parts.is_subset(&n.segments)).unwrap();
    let zero_six: Vec<&Number> = zero_six_nine.into_iter().filter(|&n| n != nine).collect();

    // The bottom bar must be the difference between the parts and the actual nine.
    let bot = **nine.segments.difference(&nine_parts).collect::<Vec<&char>>().first().unwrap();

    // The bottom left must be the difference between the eight and the nine.
    let ble = **eight.segments.difference(&nine.segments).collect::<Vec<&char>>().first().unwrap();

    // Gather the 5 parts of the zero that we know.
    let mut zero_parts: HashSet<char> = seven.segments.clone();
    zero_parts.insert(bot);
    zero_parts.insert(bot);
    zero_parts.insert(ble);
    assert_eq!(5, zero_parts.len());
    let zero = *zero_six.iter().find(|&n| zero_parts.is_subset(&n.segments)).unwrap();
    let six = *zero_six.into_iter().filter(|&n| n != zero).collect::<Vec<&Number>>().first().unwrap();

    let tle = **zero.segments.difference(&zero_parts).collect::<Vec<&char>>().first().unwrap();
    // The top left part or the middle part must be the parts of the four minus the parts of the one.
    let mut tle_mid = four.segments.difference(&one.segments).map(|c| *c).collect::<HashSet<char>>();
    tle_mid.remove(&tle);
    let mid = **tle_mid.iter().collect::<Vec<&char>>().first().unwrap();

    let mut six_parts = HashSet::new();
    six_parts.insert(bot);
    six_parts.insert(ble);
    six_parts.insert(mid);
    six_parts.insert(tle);
    six_parts.insert(top);
    let bri = **six.segments.difference(&six_parts).collect::<Vec<&char>>().first().unwrap();
    let tri = **eight.segments.difference(&six.segments).collect::<Vec<&char>>().first().unwrap();

    Display {
        top, tle, tri, mid, ble, bri, bot
    }
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

fn check_number(num_segments: i32) -> Option<i32> {
    match num_segments {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}
