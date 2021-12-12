use itertools::{max, min};
use std::cmp;
use std::fs::File;
use std::io::{BufReader, Read};

fn parse_input(filepath: &str) -> Vec<i32> {
    let file = File::open(filepath).expect("Unable to read file from path");
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    reader.read_to_string(&mut input);

    let mut result = Vec::new();

    for val in input.split(",") {
        result.push(val.parse::<i32>().expect("Unable to parse string to int."));
    }

    result
}

fn average(positions: &Vec<i32>) -> i32 {
    let total: i32 = positions
        .iter()
        .sum();
    total / (positions.len() as i32)
}

fn find_minimal_cost(positions: &Vec<i32>) -> i32 {
    let min = *min(positions.iter()).expect("Couldn't find minimum value");
    let max = *max(positions.iter()).expect("Couldn't find maximum value");

    let mut cost = calculate_cost(positions, min);
    for x in min + 1..=max {
        let tmp_cost = calculate_cost(positions, x);
        cost = cmp::min(cost, tmp_cost);
    }

    cost
}

fn calculate_cost(positions: &Vec<i32>, target: i32) -> i32 {
    positions
        .iter()
        .fold(0, |sum, pos| {
            let diff = (pos - target).abs();
            sum + (diff * (diff + 1)) / 2
        })
}

pub fn answer_1() {
    let input = parse_input("inputs/day_7.txt");
    let cost = find_minimal_cost(&input);
    println!("The minimal cost is {}", cost);
}
