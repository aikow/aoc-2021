use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer_1() {
    let graph = parse_input("inputs/day_15/input.txt");
    print_graph(&graph);

    if let Some(cost) = shortest_path(
        &graph,
        Point { row: 0, col: 0 },
        Point {
            row: graph.len() - 1,
            col: graph[0].len() - 1,
        },
    ) {
        println!("Cost is {}", cost);
    } else {
        println!("No shortest path found");
    }
}

pub fn answer_2() {
    let graph = parse_input("inputs/day_15/input.txt");
    println!("original graph");
    print_graph(&graph);

    let large_graph = create_tiled_graph(&graph, 5, 5);
    println!("Large graph: ");
    print_graph(&large_graph);

    if let Some(cost) = shortest_path(
        &large_graph,
        Point { row: 0, col: 0 },
        Point {
            row: large_graph.len() - 1,
            col: large_graph[0].len() - 1,
        },
    ) {
        println!("Cost of large graph is {}", cost);
    } else {
        println!("No shortest path found");
    }
}

fn parse_input(filepath: &str) -> Vec<Vec<i8>> {
    let file = File::open(filepath).expect("Unable to open file.");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn create_tiled_graph(graph: &Vec<Vec<i8>>, x_rows: usize, x_cols: usize) -> Vec<Vec<i8>> {
    let small_rows = graph.len();
    let small_cols = graph[0].len();
    let rows = small_rows * x_rows;
    let cols = small_cols * x_cols;
    let mut large: Vec<Vec<i8>> = (0..rows).map(|_| (0..cols).map(|_| 0).collect()).collect();

    for r in 0..rows {
        for c in 0..cols {
            let mut risk = (graph[r % small_rows][c % small_cols] as usize
                + (r / small_rows + c / small_cols));

            if risk > 9 {
                risk = risk % 9;
            };
            // println!("Risk: {} mapped to {}", risk, risk % 9);

            large[r][c] = risk as i8;
        }
    }

    large
}

fn print_graph(graph: &Vec<Vec<i8>>) {
    for row in graph {
        println!("{}", row.iter().join(""));
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.row
            .cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Point {
    fn neighbors(&self, rows: &usize, cols: &usize) -> Vec<Point> {
        let row = self.row as i32;
        let col = self.col as i32;
        let rows = *rows as i32;
        let cols = *cols as i32;
        vec![
            (row - 1, col),
            (row, col - 1),
            (row, col + 1),
            (row + 1, col),
        ]
        .into_iter()
        .filter(|(r, c)| *r >= 0 && *r < rows && *c >= 0 && *c < cols)
        .map(|(r, c)| Point {
            row: r as usize,
            col: c as usize,
        })
        .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Returns an optional cost if a path is found.
fn shortest_path(graph: &Vec<Vec<i8>>, start: Point, goal: Point) -> Option<usize> {
    let rows = graph.len();
    let cols = graph[0].len();

    let mut dist: Vec<Vec<_>> = (0..rows)
        .map(|_| (0..cols).map(|_| usize::MAX).collect())
        .collect();

    let mut pq = BinaryHeap::new();

    dist[start.row][start.col] = 0;
    pq.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = pq.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position.row][position.col] {
            continue;
        }

        for neighbor in position.neighbors(&rows, &cols) {
            let neighbor_cost = graph[neighbor.row][neighbor.col] as usize;
            let next = State {
                cost: cost + neighbor_cost,
                position: neighbor,
            };

            if next.cost < dist[neighbor.row][neighbor.col] {
                pq.push(next);
                dist[neighbor.row][neighbor.col] = next.cost;
            }
        }
    }

    None
}
