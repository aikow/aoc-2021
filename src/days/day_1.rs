use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("File not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect()
}

fn sliding_window(numbers: &Vec<i32>, window_size: usize) -> Vec<i32> {
    let mut windowed = Vec::new();
    let max_index = numbers.len() - window_size;

    let mut index = 0;
    while index <= max_index {
        let mut i: usize = 0;
        let mut average = 0;

        // print!("Inputs: ");
        while i < window_size {
            // print!("{} ", numbers[index + i]);
            average += numbers[index + i];
            i += 1;
        }

        // println!("Average: {}", average);

        windowed.push(average);

        index += 1;
    }

    return windowed;
}

fn num_increases(numbers: &Vec<i32>) -> i32 {
    let mut numbers_iter = numbers.iter();
    let mut num_increases: i32 = 0;

    if let Some(mut last_number) = numbers_iter.next() {
        for num in numbers_iter {
            if num > last_number {
                num_increases += 1;
            }
            last_number = num;
        }
    }

    return num_increases;
}

pub mod answer {
    use super::*;

    fn problem_1() {
        println!("Day 1, Problem 1");
        let input = load_from_file("inputs/1/1/day_3.txt");
        let num_increases = num_increases(&input);
        println!("Number of increases is {}", num_increases);
    }

    fn problem_2() {
        println!("Day 1, Problem 1");
        let input = load_from_file("inputs/1/1/day_3.txt");
        println!("Length is {}", input.len());
        let input = sliding_window(&input, 3);
        println!("Length is {}", input.len());
        let num_increases = num_increases(&input);
        println!("Number of increases is {}", num_increases);
    }
}