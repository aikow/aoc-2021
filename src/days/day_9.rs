use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn answer_1() {
    let matrix = parse_input("inputs/day_9.txt");
    println!("Matrix has {} rows", matrix.len());
    let low_points = find_low_points(&matrix);
    println!("Matrix has {} low points.", low_points.len());
    println!("The total risk is {}", low_points.iter().map(|p| p.1 + 1).sum::<u32>());
}

pub fn answer_2() {
    let matrix = parse_input("inputs/day_9.txt");
    println!("Matrix has {} rows", matrix.len());
    let low_points = find_low_points(&matrix);
    println!("Matrix has {} low points.", low_points.len());
    println!("The total risk is {}", low_points.iter().map(|p| p.1 + 1).sum::<u32>());
    let mut basins: Vec<u32> = low_points.iter().map(|p| find_basin_size(&matrix, *p)).collect();
    basins.sort();
    basins.reverse();
    // for basin in basins {
    //     println!("Found basin of size {}", basin);
    // }
    let answer: u32 = basins.iter().take(3).product();
    println!("The answer is {}", answer);


}

fn parse_input(filepath: &str) -> Vec<Vec<u32>> {
    let file = File::open(filepath).expect("could not open file");
    let reader = BufReader::new(file);
    reader.lines().map(|line| {
        let line = line.unwrap();
        line
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect()
    }).collect()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct LowPoint(Point, u32);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point(usize, usize);

impl Point {
    fn up(&self) -> Option<Point> {
        return if self.0 > 0 {
            Some(Point(self.0 - 1, self.1))
        } else {
            None
        };
    }

    fn down(&self, max: usize) -> Option<Point> {
        return if self.0 < max - 1 {
            Some(Point(self.0 + 1, self.1))
        } else {
            None
        };
    }

    fn left(&self) -> Option<Point> {
        return if self.1 > 0 {
            Some(Point(self.0, self.1 - 1))
        } else {
            None
        };
    }

    fn right(&self, max: usize) -> Option<Point> {
        return if self.1 < max - 1 {
            Some(Point(self.0, self.1 + 1))
        } else {
            None
        };
    }
}

fn find_low_points(matrix: &Vec<Vec<u32>>) -> HashSet<LowPoint> {
    let mut result = HashSet::new();

    for r in 0..matrix.len() {
        for c in 0..matrix[r].len() {
            let cur = matrix[r][c];
            if r > 0 {
                if let Some(up_row) = matrix.get(r - 1) {
                    if up_row[c] <= cur {
                        continue;
                    }
                }
            }
            if c > 0 {
                if let Some(left) = matrix[r].get(c - 1) {
                    if *left <= cur {
                        continue;
                    }
                }
            }
            if let Some(right) = matrix[r].get(c + 1) {
                if *right <= cur {
                    continue;
                }
            }
            if let Some(down_row) = matrix.get(r + 1) {
                if down_row[c] <= cur {
                    continue;
                }
            }
            result.insert(LowPoint(Point(r, c), matrix[r][c]));
        }
    }

    result
}

fn find_basin_size(matrix: &Vec<Vec<u32>>, start: LowPoint) -> u32 {
    let mut visited = HashSet::with_capacity(matrix.len() * matrix[0].len());
    find_basin_size_helper(matrix, &mut visited, start.0);
    return visited.len() as u32;
}

fn find_basin_size_helper(matrix: &Vec<Vec<u32>>, visited: &mut HashSet<Point>, point: Point) {
    if matrix[point.0][point.1] >= 9 { return; }
    if visited.contains(&point) { return }

    visited.insert(point);

    if let Some(up) = point.up() {
        find_basin_size_helper(matrix, visited, up);
    }
    if let Some(down) = point.down(matrix.len()) {
        find_basin_size_helper(matrix, visited, down);
    }
    if let Some(left) = point.left() {
        find_basin_size_helper(matrix, visited, left);
    }
    if let Some(right) = point.right(matrix[point.0].len()) {
        find_basin_size_helper(matrix, visited, right);
    }
}