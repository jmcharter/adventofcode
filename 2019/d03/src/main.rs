use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::{io, path::Path};

fn read_input(filename: &str) -> io::Result<String> {
    let path = Path::new(&filename);
    let input = fs::read_to_string(path)?;
    Ok(input)
}

fn calc_manhattan_dist(p1: (i32, i32), p2: &(i32, i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let x = i32::abs(x1 - x2);
    let y = i32::abs(y1 - y2);
    x + y
}

fn get_instructions_from_text(text: &str) -> Vec<String> {
    let instructions = text.split(',');
    instructions.map(|s| s.to_string()).collect()
}

fn parse_instruction(
    current_pos: (i32, i32),
    instruction: &String,
) -> Result<(i32, i32), Box<dyn Error>> {
    let (direction, value_digits) = instruction.split_at(1);
    let value: i32 = value_digits.parse()?;
    let delta = match direction {
        "U" => (0, value),
        "D" => (0, -value),
        "L" => (-value, 0),
        "R" => (value, 0),
        _ => return Err(From::from("Direction given not recognised")),
    };
    Ok((current_pos.0 + delta.0, current_pos.1 + delta.1))
}

fn get_intersections(v1: Vec<(i32, i32)>, v2: Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let set1: HashSet<_> = v1.into_iter().collect();
    let set2: HashSet<_> = v2.into_iter().collect();

    set1.intersection(&set2).cloned().collect()
}

fn get_points_between(p1: (i32, i32), p2: (i32, i32)) -> Vec<(i32, i32)> {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    if x1 == x2 {
        if y1 < y2 {
            (y1..=y2).map(|y| (x1, y)).collect()
        } else {
            (y2..=y1).map(|y| (x1, y)).collect()
        }
    } else {
        if x1 < x2 {
            (x1..=x2).map(|x| (x, y1)).collect()
        } else {
            (x2..=x1).map(|x| (x, y1)).collect()
        }
    }
}

fn interpolate_vector(vector: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut interpolated_vec: Vec<(i32, i32)> = Vec::new();
    for (i, _) in vector.iter().enumerate() {
        if i >= vector.len() - 1 {
            interpolated_vec.push(vector[i]);
            break;
        }
        let mut points_between = get_points_between(vector[i], vector[i + 1]);
        interpolated_vec.append(&mut points_between);
    }
    interpolated_vec
}

fn main() {
    let input = read_input("sample").unwrap();
    let lines = input.trim().split('\n').collect::<Vec<_>>();
    let paths: Vec<_> = lines
        .iter()
        .map(|e| get_instructions_from_text(e))
        .map(|e| {
            let mut pos = (0, 0);
            e.iter()
                .map(|x| {
                    pos = parse_instruction(pos, x).unwrap();
                    pos
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let paths: Vec<_> = paths.iter().map(interpolate_vector).collect();
    let intersections = get_intersections(paths[0].clone(), paths[1].clone());
    let distances: Vec<_> = intersections
        .iter()
        .map(|i| calc_manhattan_dist((0, 0), i))
        .collect();
    let min_dist = distances.iter().min().unwrap();
    println!("Part 01: {min_dist}");
    println!("Part 02: ??");
}
