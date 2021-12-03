use std::fs::File;
use std::io::{BufRead, BufReader};


mod problem_1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    #[derive(Debug, Copy, Clone)]
    struct SubPos {
        horizontal: i32,
        vertical: i32,
    }

    impl SubPos {
        fn new() -> SubPos {
            SubPos {
                horizontal: 0,
                vertical: 0,
            }
        }

        fn forward(&mut self, x: i32) {
            self.horizontal += x
        }

        fn up(&mut self, x: i32) {
            self.vertical -= x;
        }

        fn down(&mut self, x: i32) {
            self.vertical += x;
        }

        fn move_command(&mut self, cmd: &SubCmd) {
            match cmd {
                SubCmd::Forward(x) => self.forward(*x),
                SubCmd::Down(x) => self.down(*x),
                SubCmd::Up(x) => self.up(*x),
            }
        }

        fn product(&self) -> i32 {
            self.horizontal * self.vertical
        }

        fn move_commands(&mut self, commands: &Vec<SubCmd>) {
            for cmd in commands {
                self.move_command(cmd);
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    enum SubCmd {
        Forward(i32),
        Down(i32),
        Up(i32),
    }

    lazy_static! {
        static ref RE_FORWARD: Regex = Regex::new(r"forward\s+(?P<x>\d+)").unwrap();
        static ref RE_DOWN: Regex = Regex::new(r"down\s+(?P<x>\d+)").unwrap();
        static ref RE_UP: Regex = Regex::new(r"up\s+(?P<x>\d+)").unwrap();
    }

    impl SubCmd {
        fn parse_str(line: &str) -> Result<SubCmd, &str> {
            if let Some(caps) = RE_FORWARD.captures(line) {
                return Ok(SubCmd::Forward(caps["x"].parse::<i32>().unwrap()));
            }
            if let Some(caps) = RE_DOWN.captures(line) {
                return Ok(SubCmd::Down(caps["x"].parse::<i32>().unwrap()));
            }
            if let Some(caps) = RE_UP.captures(line) {
                return Ok(SubCmd::Up(caps["x"].parse::<i32>().unwrap()));
            }

            Err("Couldn't parse line.")
        }

        fn load_commands_from_file(file_path: &str) -> Vec<SubCmd> {
            let file = File::open(file_path).expect("File not found");
            let reader = BufReader::new(file);

            reader
                .lines()
                .map(|line| SubCmd::parse_str(&line.unwrap()).unwrap())
                .collect()
        }
    }

    pub fn problem_1() {
        let mut pos = SubPos::new();
        let commands = SubCmd::load_commands_from_file("problems/2/commands.txt");
        println!("Commands: {}", commands.len());
        pos.move_commands(&commands);
        println!("position {:?}", pos);
        println!("Multiplied: {}", pos.product());
    }
}

pub mod problem_2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use regex::Regex;

    #[derive(Debug, Copy, Clone)]
    struct SubPos {
        horizontal: i32,
        vertical: i32,
        aim: i32,
    }

    impl SubPos {
        fn new() -> SubPos {
            SubPos {
                horizontal: 0,
                vertical: 0,
                aim: 0,
            }
        }

        fn forward(&mut self, x: i32) {
            self.horizontal += x;
            self.vertical += self.aim * x;
        }

        fn up(&mut self, x: i32) {
            self.aim -= x;
        }

        fn down(&mut self, x: i32) {
            self.aim += x;
        }

        fn move_command(&mut self, cmd: &SubCmd) {
            match cmd {
                SubCmd::Forward(x) => self.forward(*x),
                SubCmd::Down(x) => self.down(*x),
                SubCmd::Up(x) => self.up(*x),
            }
        }

        fn product(&self) -> i32 {
            self.horizontal * self.vertical
        }

        fn move_commands(&mut self, commands: &Vec<SubCmd>) {
            for cmd in commands {
                self.move_command(cmd);
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    enum SubCmd {
        Forward(i32),
        Down(i32),
        Up(i32),
    }

    lazy_static! {
        static ref RE_FORWARD: Regex = Regex::new(r"forward\s+(?P<x>\d+)").unwrap();
        static ref RE_DOWN: Regex = Regex::new(r"down\s+(?P<x>\d+)").unwrap();
        static ref RE_UP: Regex = Regex::new(r"up\s+(?P<x>\d+)").unwrap();
    }

    impl SubCmd {
        fn parse_str(line: &str) -> Result<SubCmd, &str> {
            if let Some(caps) = RE_FORWARD.captures(line) {
                return Ok(SubCmd::Forward(caps["x"].parse::<i32>().unwrap()));
            }
            if let Some(caps) = RE_DOWN.captures(line) {
                return Ok(SubCmd::Down(caps["x"].parse::<i32>().unwrap()));
            }
            if let Some(caps) = RE_UP.captures(line) {
                return Ok(SubCmd::Up(caps["x"].parse::<i32>().unwrap()));
            }

            Err("Couldn't parse line.")
        }

        fn load_commands_from_file(file_path: &str) -> Vec<SubCmd> {
            let file = File::open(file_path).expect("File not found");
            let reader = BufReader::new(file);

            reader
                .lines()
                .map(|line| SubCmd::parse_str(&line.unwrap()).unwrap())
                .collect()
        }
    }

    pub fn answer() {
        let mut pos = SubPos::new();
        let commands = SubCmd::load_commands_from_file("problems/2/commands.txt");
        println!("Commands: {}", commands.len());
        pos.move_commands(&commands);
        println!("position {:?}", pos);
        println!("Multiplied: {}", pos.product());
    }
}
