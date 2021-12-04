pub mod problem_1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn read_inputs(filepath: &str) -> Vec<u16> {
        let file = File::open(filepath).expect("File not found.");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap())
            .collect()
    }

    fn find_most_common(values: &Vec<u16>) -> u16 {
        let mut result: u16 = 0;

        let len: u16 = 12;
        let mut num_ones = [0; 12];

        for bytes in values {
            for shift in 0..len {
                if bytes & (0x0001 << shift) != 0 {
                    num_ones[shift as usize] += 1;
                }
            }
        }

        for shift in 0..len {
            if num_ones[shift as usize] > values.len() / 2 {
                result |= 0x0001 << shift
            }
        }

        return result;
    }

    pub fn answer() {
        let input = read_inputs("inputs/day_3.txt");
        let most_common = find_most_common(&input);
        let least_common = !most_common & 0xFFF;
        println!(
            "Most common bytes are {:#012b}, {}",
            most_common, most_common
        );
        println!(
            "Least common bytes are {:#012b}, {}",
            least_common, least_common
        );
        println!(
            "The total power is {}",
            (most_common as u32) * (least_common as u32)
        );
    }
}

pub mod problem_2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn read_inputs(filepath: &str) -> Vec<u16> {
        let file = File::open(filepath).expect("Could not open file.");
        let reader = BufReader::new(file);
        reader
            .lines()
            .map(|line| u16::from_str_radix(&line.unwrap(), 2).unwrap())
            .collect()
    }

    fn count_ones(values: &Vec<u16>, pos: u16) -> i32 {
        let mask: u16 = 0x0001 << pos;
        values.iter().fold(
            0,
            move |count, val| {
                if val & mask != 0 {
                    count + 1
                } else {
                    count
                }
            },
        )
    }

    fn oxygen_generator_rating(values: &Vec<u16>) -> u16 {
        let mut filtered = values.clone();
        let mut pos: u16 = 11;

        while filtered.len() > 1 {
            let mask = 0x0001 << pos;
            let num_ones = count_ones(&filtered, pos);
            println!(
                "{} number of {} have ones at position {}",
                num_ones,
                filtered.len(),
                pos
            );

            if num_ones >= (filtered.len() as i32 + 1) / 2 {
                println!("Keeping numbers with 1");
                // more or same ones than zeros
                filtered = filtered
                    .into_iter()
                    .filter(|&val| val & mask != 0)
                    .collect();
            } else {
                println!("Keeping numbers with 0");
                filtered = filtered
                    .into_iter()
                    .filter(|&val| val & mask == 0)
                    .collect();
            }
            if pos == 0 {
                pos = 12;
            }
            pos -= 1;
        }

        filtered[0]
    }

    fn co2_scrubbing_rate(values: &Vec<u16>) -> u16 {
        let mut filtered = values.clone();
        let mut pos: u16 = 11;

        while filtered.len() > 1 {
            let mask = 0x0001 << pos;
            let num_ones = count_ones(&filtered, pos);
            println!(
                "{} number of {} have ones at position {}",
                num_ones,
                filtered.len(),
                pos
            );

            if num_ones >= (filtered.len() as i32 + 1) / 2 {
                println!("Keeping numbers with 0");
                // If there are fewer zeros than ones
                filtered = filtered
                    .into_iter()
                    .filter(|&val| val & mask == 0)
                    .collect();
            } else {
                println!("Keeping numbers with 1");
                // If there are fewer ones than zeros
                filtered = filtered
                    .into_iter()
                    .filter(|&val| val & mask != 0)
                    .collect();
            }

            if pos == 0 {
                pos = 12;
            }
            pos -= 1;
        }

        filtered[0]
    }

    pub fn answer() {
        let inputs = read_inputs("inputs/day_3.txt");
        let ogr = oxygen_generator_rating(&inputs);
        let csr = co2_scrubbing_rate(&inputs);
        println!("Oxygen generator rating {:012b}, {}", ogr, ogr);
        println!("CO2 scrubber rating {:012b}, {}", csr, csr);
        println!("Life support rating {}", (ogr as u32) * (csr as u32));
    }
}
