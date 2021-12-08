use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    /// Shorthand constructor
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Line {
        Line(Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
    }

    fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    fn points(&self) -> Vec<Point> {
        let mut results = Vec::new();
        if self.is_horizontal() {
            let x1 = min(self.0.x, self.1.x);
            let x2 = max(self.0.x, self.1.x);
            for x in x1..=x2 {
                // Since line is horizontal, all y values are the same.
                let point = Point { x, y: self.0.y };
                results.push(point);
            }
        } else if self.is_vertical() {
            let y1 = min(self.0.y, self.1.y);
            let y2 = max(self.0.y, self.1.y);
            for y in y1..=y2 {
                // Since line is horizontal, all y values are the same.
                let point = Point { x: self.0.x, y };
                results.push(point);
            }
        } else {
            // Line is diagonal.

            // Figure out slope and start point.
            // traverse
        }

        results
    }
}

lazy_static! {
    static ref REGEX_LINE: Regex = Regex::new(r"(\d+),(\d+)\s+->\s+(\d+),(\d+)").unwrap();
}

fn parse_input(file_path: &str) -> Vec<Line> {
    let file = File::open(file_path).expect("Failed to open file.");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let caps = REGEX_LINE.captures(&line).unwrap();
            Line(
                Point {
                    x: caps.get(1).unwrap().as_str().parse().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse().unwrap(),
                },
                Point {
                    x: caps.get(3).unwrap().as_str().parse().unwrap(),
                    y: caps.get(4).unwrap().as_str().parse().unwrap(),
                },
            )
        })
        .collect()
}

fn find_intersections(lines: &Vec<Line>) -> i32 {
    // Store a count for each point how many times a line has intersected with that point.
    let mut intersections: HashMap<Point, i32> = HashMap::new();

    for line in lines {
        if line.is_horizontal() {
            let x1 = min(line.0.x, line.1.x);
            let x2 = max(line.0.x, line.1.x);
            for x in x1..=x2 {
                // Since line is horizontal, all y values are the same.
                let point = Point { x, y: line.0.y };
                let count = intersections.entry(point).or_insert(0);
                *count += 1;
            }
        } else if line.is_vertical() {
            let y1 = min(line.0.y, line.1.y);
            let y2 = max(line.0.y, line.1.y);
            for y in y1..=y2 {
                // Since line is horizontal, all y values are the same.
                let point = Point { x: line.0.x, y };
                let count = intersections.entry(point).or_insert(0);
                *count += 1;
            }
        } else {
            let (p1, p2) = if line.0.x < line.1.x {
                (line.0, line.1)
            } else {
                (line.1, line.0)
            };
            let dist = p2.x - p1.x;
            let slope = if p1.y < p2.y { 1 } else { -1 };
            println!("Going from {:?} to {:?}", p1, p2);

            for d in 0..=dist {
                let point = Point {
                    x: p1.x + d,
                    y: p1.y + d * slope,
                };
                let count = intersections.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut total = 0;
    for count in intersections.values() {
        if *count >= 2 {
            total += 1;
        }
    }

    return total;
}

pub fn answer_1() {
    let mut lines = parse_input("inputs/day_5.txt");
    println!("Found {} lines.", lines.len());
    println!();

    // Keep only horizontal or vertical lines.
    horizontal_or_vertical(&mut lines);
    println!("{} lines are horizontal or vertical", lines.len());

    let intersections = find_intersections(&lines);
    println!("Found {} intersections", intersections);
}

pub fn answer_2() {
    let lines = parse_input("inputs/day_5.txt");
    println!("Found {} lines.", lines.len());
    println!();

    let intersections = find_intersections(&lines);
    println!("Found {} intersections", intersections);
}

fn horizontal_or_vertical(lines: &mut Vec<Line>) {
    lines.retain(|&line| line.is_horizontal() || line.is_vertical())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_example() -> Vec<Line> {
        return vec![
            Line::new(0, 9, 5, 9),
            Line::new(8, 0, 0, 8),
            Line::new(9, 4, 3, 4),
            Line::new(2, 2, 2, 1),
            Line::new(7, 0, 7, 4),
            Line::new(6, 4, 2, 0),
            Line::new(0, 9, 2, 9),
            Line::new(3, 4, 1, 4),
            Line::new(0, 0, 8, 8),
            Line::new(5, 5, 8, 2),
        ];
    }

    #[test]
    fn check_input() {
        let lines = parse_input("inputs/day_5_test.txt");
        let ref_lines = create_example();
        for (line, ref_line) in lines.iter().zip(&ref_lines) {
            assert_eq!(line, ref_line);
        }
    }

    #[test]
    fn check_filter() {
        let mut lines = create_example();
        horizontal_or_vertical(&mut lines);
        assert_eq!(6, lines.len());
    }

    #[test]
    fn check_intersections() {
        let mut lines = create_example();
        horizontal_or_vertical(&mut lines);
        let intersections = find_intersections(&lines);
        assert_eq!(5, intersections);
    }

    #[test]
    fn check_all_intersections() {
        let lines = create_example();
        let intersections = find_intersections(&lines);
        assert_eq!(12, intersections);
    }
}
