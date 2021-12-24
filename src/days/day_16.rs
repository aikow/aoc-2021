use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::Path;
use itertools::Itertools;

pub fn answer_1() {
    let input = parse_input("inputs/day_16/test.txt");
    println!("{}", input);

    let (version, packet_type) = get_version_type(&input);
    println!("Packet (version: {}, type: {})", version, packet_type);
}

pub fn answer_2() {}

fn parse_input<P>(filepath: P) -> String
where
    P: AsRef<Path>,
{
    let mut file = File::open(filepath.as_ref()).expect("Unable to open filepath");

    // Read contents of file to string.
    let mut input = String::new();
    file.read_to_string(&mut input);

    input
        .chars()
        .flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap()).chars().collect::<Vec<char>>()
        })
        .collect()
}

enum PacketType {
    Literal(i32),
    Operator(Vec<Packet>),
}

struct Packet {
    version: u8,
    kind: PacketType,
}

fn get_version_type<P>(packet: P) -> (u8, u8) where P: AsRef<str> {
    let version = u8::from_str_radix(&packet.as_ref()[0..3], 2).unwrap();
    let packet_type = u8::from_str_radix(&packet.as_ref()[3..6], 2).unwrap();

    (version, packet_type)
}

fn parse_packet<S>(slice: S) -> Packet where S: AsRef<str> {
    let (version, kind) = get_version_type(slice);

    let kind = match kind {
        4 => {
            let mut value: i32 = 0;
            let read_len = parse_literal(&slice[6..], &mut value);
            PacketType::Literal(value)
        }
        _ => {

        }
    };

    Packet { version, kind }
}

fn parse_type_1<S>(packet: S) -> Packet where S: AsRef<str> {}

fn parse_type_2<S>(packet: S) -> Packet where S: AsRef<str> {}

fn parse_literal<S>(slice: S, number: &mut i32) -> i32 where S: AsRef<str> {
    let mut iter = slice.as_ref().chars().chunks(5);
    let mut number= String::new();

    loop {
        let chunks = iter.next().iter();
        let first = chunks.next();
        number.add(chunks.collect());

        // Last chunk of 5
        if chunks[0] == 0 {
            break;
        }
    }

    0
}