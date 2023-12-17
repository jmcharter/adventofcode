use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file1 = fs::File::open(filename)?;
    let file2 = fs::File::open(filename)?;
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    println!("Part one: {}", process_part_one(reader1));
    println!("Part two: {}", process_part_two(reader2));
    Ok(())
}

fn parse_data<R: Read>(reader: BufReader<R>) -> Vec<String> {
    let lines: Vec<String> = reader.lines().flatten().collect();
    let line = &lines[0];
    let steps = line.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
    steps
}

fn hash_input(input: &str) -> u32 {
    let mut value: u32 = 0;
    for char in input.chars() {
        let ascii_code = char as u8;
        value += ascii_code as u32;
        value *= 17;
        value %= 256;
    }
    value
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let steps = parse_data(reader);
    let hashes: Vec<_> = steps.iter().map(|step| hash_input(step)).collect();
    hashes.iter().sum::<u32>()
}

#[derive(Debug, Clone)]
enum Operation {
    Remove,
    Add,
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    focal_length: Option<u8>,
    hash: u32,
    operation: Operation,
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
    let mut lens_map: HashMap<u32, Vec<Lens>> = HashMap::new();
    let data = parse_data(reader);
    let lenses: Vec<Lens> = data
        .iter()
        .map(|lens| {
            let operation = match lens.chars().find(|x| !x.is_alphanumeric()) {
                Some('=') => Operation::Add,
                Some('-') => Operation::Remove,
                _ => panic!("Invalid input or poorly parsed"),
            };
            let len_split: Vec<_> = lens.split(|ch: char| !ch.is_alphanumeric()).collect();
            let label = len_split[0];
            let focal_length = match operation {
                Operation::Add => Some(len_split[1].parse::<u8>().expect("parsable to u8")),
                Operation::Remove => None,
            };
            let hash = hash_input(label);
            Lens {
                label: label.to_string(),
                focal_length,
                hash,
                operation,
            }
        })
        .collect();

    for lens in &lenses {
        match lens.operation {
            Operation::Remove => {
                if let Some(value) = lens_map.get_mut(&lens.hash) {
                    if let Some(pos) = value.iter().position(|x| x.label == lens.label) {
                        value.remove(pos);
                    }
                }
            }
            Operation::Add => {
                let value = lens_map.entry(lens.hash).or_default();
                if let Some(pos) = value.iter().position(|x| x.label == lens.label) {
                    value.remove(pos);
                    value.insert(pos, lens.clone())
                } else {
                    value.push(lens.clone())
                }
            }
        }
    }
    lens_map
        .iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(move |(i, e)| {
                    (k + 1)
                        * (i as u32 + 1)
                        * e.focal_length.expect("None variants should not be present") as u32
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(1320, process_part_one(BufReader::new(input_bytes)));
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(145, process_part_two(BufReader::new(input_bytes)));
    }
}
