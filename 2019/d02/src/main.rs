use std::fs;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum OpCode {
    Add = 1,
    Multiply = 2,
    Halt = 99,
}

impl OpCode {
    fn from_u32(value: u32) -> Option<Self> {
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

fn part_one(input: &String, noun: u32, verb: u32) -> u32 {
    let mut integers: Vec<u32> = input
        .trim()
        .split(',')
        .map(|e| e.parse().unwrap())
        .collect();
    integers[1] = noun;
    integers[2] = verb;
    let mut position = 0;
    while position < integers.len() {
        let op_code = OpCode::from_u32(integers[position]).unwrap();
        if op_code == OpCode::Halt {
            break;
        }

        // Get the value of integers at index n, and convert that value to an index (usize)
        let in1_i: usize = integers[position + 1] as usize;
        let in2_i: usize = integers[position + 2] as usize;
        let out_i: usize = integers[position + 3] as usize;
        let in1: u32 = integers[in1_i];
        let in2: u32 = integers[in2_i];

        let result: u32 = match op_code {
            OpCode::Add => in1 + in2,
            OpCode::Multiply => in1 * in2,
            _ => break,
        };

        integers[out_i] = result;
        position += 4
    }
    integers[0]
}

fn part_two(input: &String) -> u32 {
    for noun in 0..99 {
        for verb in 0..99 {
            let result = part_one(&input, noun, verb);
            if result == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn main() {
    let input = read_input().unwrap();
    println!("Part one: {:?}", part_one(&input, 12, 2));
    println!("Part two: {:?}", part_two(&input));
}
