use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LANTERN_FISH_AGE: i8 = 6;
const NEW_LANTERN_FISH_AGE: i8 = 8;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct LanternFish(i8);

impl LanternFish {
    fn live(&mut self) -> Option<LanternFish> {
        self.0 -= 1;
        if self.0 < 0 {
            self.0 = LANTERN_FISH_AGE;
            return Some(LanternFish(NEW_LANTERN_FISH_AGE));
        }

        None
    }
}

///
/// Returns the number of lantern fish in the school
fn simulate(school: &mut Vec<LanternFish>, days: i32) {
    for day in 0..days {
        let mut babies: Vec<LanternFish> =
            school.iter_mut().filter_map(|fish| fish.live()).collect();

        school.append(&mut babies);
        // let fmt_school = school
        //     .iter()
        //     .map(|fish| fish.age)
        //     .join(",");
        println!("After {:2} days: {}", day, school.len());
    }
}

fn parse_input(filepath: &str) -> Vec<LanternFish> {
    let inputs = std::fs::read_to_string(filepath).expect("Could not read file.");
    inputs
        .trim()
        .split(",")
        .map(|line| {
            let age = line.parse::<i8>().unwrap();
            LanternFish(age)
        })
        .collect()
}

mod smort {
    use crate::days::day_6::{LanternFish, LANTERN_FISH_AGE, NEW_LANTERN_FISH_AGE};
    use std::collections::HashMap;

    pub fn parse_input(filepath: &str) -> HashMap<i8, u128> {
        let mut school = HashMap::new();
        let inputs = std::fs::read_to_string(filepath).expect("Could not read from file.");

        for age_str in inputs.trim().split(",") {
            let age = age_str.parse::<i8>().unwrap();
            let count = school.entry(age).or_insert(0);
            *count += 1;
        }

        school
    }

    pub fn simulate(school: &mut HashMap<i8, u128>, days: i32) {
        for _day in 0..days {
            let num_babies = school.remove(&0).unwrap_or(0);
            for age in 0..NEW_LANTERN_FISH_AGE {
                let new_group = *school.get(&(age + 1)).unwrap_or(&0);
                school.insert(age, new_group);
            }
            match school.get_mut(&LANTERN_FISH_AGE) {
                Some(count) => {
                    *count += num_babies;
                }
                None => {
                    school.insert(LANTERN_FISH_AGE, num_babies);
                }
            }
            school.insert(NEW_LANTERN_FISH_AGE, num_babies);
        }
    }
}

pub fn answer_1() {
    let mut school = parse_input("inputs/day_6.txt");
    let days = 256;
    simulate(&mut school, days);
    println!("There are {} fish after {} days", school.len(), days);
}

pub fn answer_2() {
    let mut school = smort::parse_input("inputs/day_6.txt");
    let days = 256;
    smort::simulate(&mut school, days);

    let num_fish: u128 = school.iter().map(|(_age, count)| count).sum();

    println!("There are {} fish after {} days", num_fish, days);
}
