use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl OpCode {
    fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(Self::Add),
            2 => Some(Self::Multiply),
            99 => Some(Self::Halt),
            _ => None,
        }
    }
}

fn read_input() -> std::io::Result<String> {
    let path = Path::new("input");
    let input = fs::read_to_string(path)?;
    Ok(input)
}

fn main() {
    let input = read_input().unwrap();
    let integers: Vec<&str> = input.split(',').collect();
    let mut position = 0;
    while let Some(op_code) = OpCode::from_i32(integers[position].parse::<i32>().unwrap()) {
        if op_code == OpCode::Halt {
            break;
        }
        println!("{:?}", op_code);
        position += 4
    }
}
