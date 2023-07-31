use std::fs;
use std::num::ParseIntError;
use std::{io, path::Path};

fn read_input(filename: &str) -> io::Result<String> {
    let path = Path::new(&filename);
    let input = fs::read_to_string(path)?;
    Ok(input)
}

fn calc_manhattan_dist(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let x = i32::abs(x1 - y1);
    let y = i32::abs(x2 - y2);
    x + y
}

fn get_instructions_from_text(text: &str) -> std::str::Split<char> {
    let instructions = text.split(',');
    instructions
}

fn parse_instruction(instruction: &str) -> Result<(i32, i32), ParseIntError> {
    let dir = instruction.chars().next()?;
    let value = instruction[1..];
}

fn main() {
    let input = read_input("sample").unwrap();
    let lines = input.split("\n").collect::<Vec<_>>();
    let instructions: Vec<_> = get_lines_from_text(lines[0]).collect();
    println!("{:?}", instructions);
}
