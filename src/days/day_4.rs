pub mod problem_1 {
    use itertools::Itertools;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[derive(Copy, Clone)]
    struct Field {
        value: i32,
        checked: bool,
    }

    struct Board {
        rows: usize,
        cols: usize,
        board: Vec<Field>,
    }

    impl Clone for Board {
        fn clone(&self) -> Self {
            return Board {
                rows: self.rows,
                cols: self.cols,
                board: self.board.clone(),
            };
        }
    }

    impl Board {
        fn new(rows: usize, cols: usize, values: Vec<i32>) -> Result<Board, &'static str> {
            if values.len() != rows * cols {
                return Err("The length of the values must be equal to the rows times cols.");
            }

            let board = values
                .into_iter()
                .map(|value| Field {
                    value,
                    checked: false,
                })
                .collect();

            Ok(Board { rows, cols, board })
        }

        fn add_value(&mut self, value: i32) {
            let mut iter = self.board.iter_mut();

            if let Some(field) = iter.find(|f| f.value == value) {
                field.checked = true;
            }
        }

        fn check_won(&self) -> bool {
            if self
                .board
                .chunks(self.cols)
                .fold(false, |res, fields| match res {
                    true => true,
                    false => fields.iter().fold(true, |res, field| match res {
                        true => field.checked,
                        false => false,
                    }),
                })
            {
                return true;
            }

            for c in 0..self.cols {
                let mut result = true;
                for r in 0..self.rows {
                    if let Some(field) = self.board.get(r * self.cols + c) {
                        if !field.checked {
                            result = false;
                        }
                    }
                }

                if result {
                    return true;
                }
            }

            return false;
        }
    }

    fn parse_input(filepath: &str) -> Result<(Vec<Board>, Vec<i32>), &'static str> {
        let file = File::open(filepath).map_err(|err| "Could not open file")?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // Read the called numbers into a vector.
        let numbers: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(",")
            .map(|s| {
                // print!("{} ", s);
                s.parse::<i32>().unwrap()
            })
            .collect();

        // List of boards.
        let mut boards: Vec<Board> = Vec::new();

        // Read the lines containing the boards into a vector.
        println!("Parsing numbers...");
        let values: Vec<String> = lines
            .into_iter()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .collect();

        for val in &values {
            println!("{}", val);
        }

        println!("Parsing boards...");
        for chunk in values.chunks(5) {
            let values: Vec<i32> = chunk
                .iter()
                .flat_map(|line| line.split_whitespace())
                .map(|s| {
                    // print!("{} ", s);
                    s.parse::<i32>().unwrap()
                })
                .collect();
            // println!("\rFound {} values for board.", values.len());

            match Board::new(5, 5, values) {
                Ok(board) => boards.push(board),
                _ => println!("Warning skipping board."),
            }
        }

        Ok((boards, numbers))
    }

    pub fn answer() {
        let (mut boards, numbers) = parse_input("inputs/day_4.txt").expect("Unable to read file.");
        println!(
            "Found {} boards and {} numbers",
            boards.len(),
            numbers.len()
        );
        'number: for number in numbers {
            for board in &mut boards {
                board.add_value(number);
                if board.check_won() {
                    let total: i32 = board
                        .board
                        .iter()
                        .filter(|&field| !field.checked)
                        .map(|field| field.value)
                        .sum();
                    let score = total * number;
                    println!("Board won on number {}. The score was {}", number, score);
                    break 'number;
                }
            }
        }
    }
    pub fn answer_2() {
        let (mut boards, numbers) = parse_input("inputs/day_4.txt").expect("Unable to read file.");
        println!(
            "Found {} boards and {} numbers",
            boards.len(),
            numbers.len()
        );
        let mut last_board: Option<Board> = None;

        for number in numbers {
            for board in &mut boards {
                board.add_value(number);
                if board.check_won() {
                    last_board = Some(board.clone());
                }
            }

            // Remove all boards which have won
            boards = boards
                .into_iter()
                .filter(|board| !board.check_won())
                .collect();

            if boards.is_empty() {
                let total: i32 = last_board
                    .unwrap()
                    .board
                    .iter()
                    .filter(|&field| !field.checked)
                    .map(|field| field.value)
                    .sum();

                let score = total * number;
                println!("Board won on number {}. The score was {}", number, score);
                break;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn horizontal_win() {
            let mut board = Board::new(3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
            board.add_value(0);
            assert!(!board.check_won());
            board.add_value(1);
            assert!(!board.check_won());
            board.add_value(9);
            assert!(!board.check_won());

            // Win condition
            board.add_value(2);
            assert!(board.check_won());
        }

        #[test]
        fn vertical_win() {
            let mut board = Board::new(3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
            board.add_value(0);
            assert!(!board.check_won());
            board.add_value(3);
            assert!(!board.check_won());

            // Win condition
            board.add_value(6);
            assert!(board.check_won());
        }

        #[test]
        fn no_win() {
            let mut board = Board::new(3, 3, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
            board.add_value(0);
            board.add_value(1);
            board.add_value(3);
            board.add_value(5);
            board.add_value(8);
            board.add_value(9);
            assert!(!board.check_won());
        }
    }
}
