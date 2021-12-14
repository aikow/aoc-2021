use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn answer_1() {
    let (mut polymer, rules) = parse_input("inputs/day_14/input.txt").unwrap();
    println!("Polymer: {}, Rules: {}", polymer, rules.len());

    for step in 1..=10 {
        polymer = add_stuff(&polymer, &rules);
        // println!("After step {}: {}", step, polymer);
        println!("After step {}", step);
    }

    let mut freq: HashMap<char, u128> = HashMap::new();
    for c in polymer.chars() {
        let entry = freq.entry(c).or_insert(0);
        *entry += 1;
    }
    let mut freq: Vec<_> = freq.into_iter().collect();
    freq.sort_by(|x, y| x.1.cmp(&y.1));

    let x1 = freq[freq.len() - 1];
    let x2 = freq[0];
    println!("Most frequent {}, {}", x1.0, x1.1);
    println!("Least frequent {}, {}", x2.0, x2.1);
    println!("Difference is {}", x1.1 - x2.1);
}

pub fn answer_2() {
    let (mut polymer, rules) = parse_input("inputs/day_14/input.txt").unwrap();
    let mut polymer_dict: HashMap<String, u128> = HashMap::new();

    for index in 1..polymer.len() {
        let key = String::from(&polymer[index - 1..index + 1]);
        let entry = polymer_dict.entry(key).or_insert(0);
        *entry += 1;
    }

    let mut temp: HashMap<String, u128>;
    let mut freq: HashMap<char, u128> = HashMap::new();

    for c in polymer.chars() {
        let e = freq.entry(c).or_insert(0);
        *e += 1;
    }

    for step in 1..=40 {
        temp = HashMap::new();
        for (k, v) in &polymer_dict {
            let res = rules[k];
            let e = freq.entry(res).or_insert(0);
            *e += v;

            let mut k1 = String::from(k.chars().next().unwrap());
            k1.push(res);
            let e1 = temp.entry(k1).or_insert(0);
            *e1 += v;

            let mut k2 = String::from(res);
            k2.push(k.chars().skip(1).next().unwrap());
            let e2 = temp.entry(k2).or_insert(0);
            *e2 += v;
        }
        polymer_dict = temp;
    }

    let mut freq: Vec<_> = freq.into_iter().collect();
    freq.sort_by(|x, y| x.1.cmp(&y.1));

    let x1 = freq[freq.len() - 1];
    let x2 = freq[0];
    println!("Most frequent {}, {}", x1.0, x1.1);
    println!("Least frequent {}, {}", x2.0, x2.1);
    println!("Difference is {}", x1.1 - x2.1);
}

fn add_stuff(start_polymer: &String, rules: &HashMap<String, char>) -> String {
    let mut polymer = String::new();
    let mut initial_chars = start_polymer.chars();
    polymer.push(initial_chars.next().unwrap());

    for index in 1..start_polymer.len() {
        let pair = &start_polymer[index - 1..index + 1];
        polymer.push(rules[pair]);
        polymer.push(initial_chars.next().unwrap());
    }

    polymer
}

fn parse_input(filepath: &str) -> Result<(String, HashMap<String, char>), &str> {
    let file = File::open(filepath).map_err(|_e| "Unable to open file")?;
    let mut reader = BufReader::new(file);

    // Read polymer from first line.
    let mut polymer = String::new();
    reader
        .read_line(&mut polymer)
        .map_err(|_e| "Unable to read first line")?;
    polymer.retain(|c| !c.is_whitespace());

    // Read the rules from third line onward.
    let mut rules = HashMap::new();
    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let split: Vec<_> = line.split(" -> ").collect();
        let pair = String::from(split[0]);
        let mut res = split[1].chars().next().unwrap();
        rules.insert(pair, res);
    }

    Ok((polymer, rules))
}
