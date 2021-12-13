use itertools::{max, Itertools};
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer_1() {
    // let (mut points, folds) = parse_input("inputs/day_13/test.txt");
    let (mut points, folds) = parse_input("inputs/day_13/input.txt");
    println!("Found {} points and {} folds", points.len(), folds.len());
    fold_points(&mut points, &folds);
    println!("Now there are {} points", points.len());
    display_points(&points);
}

pub fn answer_2() {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Axis {
    XAxis(i32),
    YAxis(i32),
}

fn parse_input(filepath: &str) -> (HashSet<Point>, Vec<Axis>) {
    lazy_static! {
        static ref FOLD_ALONG: Regex =
            Regex::new(r"fold along (?P<axis>[xy])=(?P<value>\d+)").unwrap();
        static ref POINT: Regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    }

    let file = File::open(filepath).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut points: HashSet<Point> = HashSet::new();
    let mut folds: Vec<Axis> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(caps) = POINT.captures(&line) {
            points.insert(Point(
                caps["x"].parse::<i32>().unwrap(),
                caps["y"].parse::<i32>().unwrap(),
            ));
        } else if let Some(caps) = FOLD_ALONG.captures(&line) {
            if caps["axis"] == *"y" {
                folds.push(Axis::XAxis(caps["value"].parse::<i32>().unwrap()));
            } else {
                folds.push(Axis::YAxis(caps["value"].parse::<i32>().unwrap()));
            }
        }
    }

    (points, folds)
}

fn fold_points(points: &mut HashSet<Point>, folds: &Vec<Axis>) {
    for axis in folds {
        match axis {
            Axis::XAxis(y) => {
                *points = points
                    .iter()
                    .map(|p| {
                        return if p.1 > *y {
                            let new_y = *y - (p.1 - *y);
                            Point(p.0, new_y)
                        } else {
                            *p
                        };
                    })
                    .collect();
            }
            Axis::YAxis(x) => {
                *points = points
                    .iter()
                    .map(|p| {
                        return if p.0 > *x {
                            let new_x = *x - (p.0 - *x);
                            Point(new_x, p.1)
                        } else {
                            *p
                        };
                    })
                    .collect();
            }
        }
        println!("{}", points.len());
    }
}

fn display_points(points: &HashSet<Point>) {
    let width = max(points.iter().map(|p| p.0)).unwrap();
    let height = max(points.iter().map(|p| p.1)).unwrap();

    for h in 0..=height {
        for w in 0..=width {
            if points.contains(&Point(w, h)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
