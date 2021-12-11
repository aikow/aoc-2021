use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub fn answer_1() {
    // let mut grid = parse_input("inputs/test/day_11_test.txt");
    let mut grid = parse_input("inputs/day_11.txt");
    println!("Initial:");
    print_grid(&grid);
    println!();

    let steps = 100;
    let mut flashes = 0;

    for k in 1..=steps {
        flashes += step(&mut grid);
        println!("Step {}:", k);
        print_grid(&grid);
        println!();
    }

    println!("Total flashes: {}", flashes);
}

pub fn answer_2() {
    // let mut grid = parse_input("inputs/test/day_11_test.txt");
    let mut grid = parse_input("inputs/day_11.txt");
    println!("Initial:");
    print_grid(&grid);
    println!();

    let mut steps = 1;

    loop {
        if step(&mut grid) == 100 {
            break;
        };
        steps += 1;
    }

    println!("Step {}:", steps);
}


fn parse_input(filepath: &str) -> Vec<Vec<u32>> {
    let file = File::open(filepath).expect("Unable to open file!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in grid {
        for octopus in row {
            print!("{:>2} ", octopus);
        }
        println!();
    }
    println!();
}

const OCTOPUS_THRESHOLD: u32 = 9;

#[derive(Eq, PartialEq, Hash)]
struct Octopus<'level>(&'level mut u32, Point);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point(usize, usize);

//noinspection DuplicatedCode
impl Point {
    fn neighbors(&self, rows: &usize, cols: &usize) -> Vec<Point> {
        let row = self.0 as i32;
        let col = self.1 as i32;
        let rows = *rows as i32;
        let cols = *cols as i32;
        vec![
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
        .into_iter()
        .filter(|(r, c)| *r >= 0 && *r < rows && *c >= 0 && *c < cols)
        .map(|(r, c)| Point(r as usize, c as usize))
        .collect()
    }
}

///
/// Returns the number of octopi that blinked.
fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            grid[row][col] += 1;
        }
    }

    let mut queue: VecDeque<Point> = grid
        .iter_mut()
        .enumerate()
        .flat_map(move |(r, row)| {
            row.iter_mut()
                .enumerate()
                .filter(|(c, o)| **o > OCTOPUS_THRESHOLD)
                .map(move |(c, o)| Point(r, c))
        })
        .collect();

    let rows = 10;
    let cols = 10;

    let mut flashed = 0;
    while let Some(point) = queue.pop_front() {
        assert!(grid[point.0][point.1] > OCTOPUS_THRESHOLD);
        // println!("({}, {}), {}", point.0, point.1, queue.iter().map(|p| format!("({}, {})", p.0, p.1)).join(", "));

        // print_grid(grid);

        flashed += 1;
        grid[point.0][point.1] = 0;
        let neighbors: Vec<Point> = point
            .neighbors(&rows, &cols)
            .into_iter()
            .filter(|&Point(r, c)| grid[r][c] != 0 && grid[r][c] <= OCTOPUS_THRESHOLD)
            .collect();

        for Point(r,c) in neighbors {
            grid[r][c] += 1;
            if grid[r][c] > OCTOPUS_THRESHOLD {
                // println!("\tAdded point ({}, {}) with level {}", r, c, grid[r][c]);
                queue.push_back(Point(r,c));
            }
        }
    }

    flashed
}
