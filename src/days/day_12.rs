use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::fs::File;
use itertools::Itertools;

#[derive(Debug)]
struct Cave {
    connected: Vec<usize>,
    large: bool,
}

pub fn answer_1() {
    let file = File::open("inputs/day_12.txt").expect("Failed to read file");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let mut cave_indexes = HashMap::new();
    let mut caves = Vec::new();

    for line in &lines {
        let words: Vec<_> = line.split('-').collect();
        for &word in &words {
            if !cave_indexes.contains_key(word) {
                cave_indexes.insert(word, caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: word.chars().next().unwrap().is_ascii_uppercase(),
                });
            }
        }

        let a = cave_indexes[words[0]];
        let b = cave_indexes[words[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = cave_indexes["start"];
    let end = cave_indexes["end"];

    println!("{}", count_paths(&caves, &mut vec![false; caves.len()], start, end));
}

fn count_paths(caves: &Vec<Cave>, visited: &mut Vec<bool>, pos: usize, end: usize) -> u16 {
    let cave = &caves[pos];

    if pos == end {
        1
    } else if visited[pos] && !cave.large {
        0
    } else {
        visited[pos] = true;
        let paths = cave.connected.iter().map(|&new_pos| {
            count_paths(caves, visited, new_pos, end)
        }).sum();
        visited[pos] = false;
        paths
    }
}

// fn parse_input() -> (HashMap<&str, usize>, Vec<Cave>) {
//     let file = File::open("inputs/day_12.txt").expect("Failed to read file");
//     let reader = BufReader::new(file);
//     let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
//
//     let mut cave_indexes = HashMap::new();
//     let mut caves = Vec::new();
//
//     for line in &lines {
//         let words: Vec<_> = line.split('-').collect();
//         for &word in &words {
//             if !cave_indexes.contains_key(word) {
//                 cave_indexes.insert(word, caves.len());
//
//                 caves.push(Cave {
//                     connected: Vec::new(),
//                     large: word.chars().next().unwrap().is_ascii_uppercase(),
//                 });
//             }
//         }
//
//         let a = cave_indexes[words[0]];
//         let b = cave_indexes[words[1]];
//         caves[a].connected.push(b);
//         caves[b].connected.push(a);
//     }
//     (cave_indexes, caves)
// }

pub fn answer_2() {
    let file = File::open("inputs/day_12.txt").expect("Failed to read file");
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    let mut cave_indexes = HashMap::new();
    let mut caves = Vec::new();

    for line in &lines {
        let words: Vec<_> = line.split('-').collect();
        for &word in &words {
            if !cave_indexes.contains_key(word) {
                cave_indexes.insert(word, caves.len());

                caves.push(Cave {
                    connected: Vec::new(),
                    large: word.chars().next().unwrap().is_ascii_uppercase(),
                });
            }
        }

        let a = cave_indexes[words[0]];
        let b = cave_indexes[words[1]];
        caves[a].connected.push(b);
        caves[b].connected.push(a);
    }

    let start = cave_indexes["start"];
    let end = cave_indexes["end"];

    println!("{}", count_multi_paths(&caves, &mut vec![0; caves.len()], false, start, start, end));
}


fn count_multi_paths(caves: &Vec<Cave>, visited: &mut Vec<u8>, multi_visited: bool, pos: usize, start: usize, end: usize) -> u32 {
    let cave = &caves[pos];
    let second_visit = visited[pos] >= 1 && !cave.large;

    if pos == end {
        1
    } else if second_visit && (multi_visited || pos == start) {
        0
    } else {
        let multi_visited = multi_visited || second_visit;
        visited[pos] += 1;
        let paths = cave.connected.iter().map(|&new_pos| {
            count_multi_paths(caves, visited, multi_visited, new_pos, start, end)
        }).sum();
        visited[pos] -= 1;
        paths
    }
}