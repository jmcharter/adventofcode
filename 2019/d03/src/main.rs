<<<<<<< HEAD
use std::collections::HashSet;
=======
use std::collections::HashMap;
>>>>>>> 6f3762551953da97a1f01210d755fab7aac651fb
use std::error::Error;
use std::fs;
use std::{io, path::Path};

fn read_input(filename: &str) -> io::Result<String> {
    let path = Path::new(&filename);
    let input = fs::read_to_string(path)?;
    Ok(input)
}

fn calc_manhattan_dist(p1: (i32, i32, i32), p2: (&(i32, i32), &i32)) -> i32 {
    let (x1, y1, _) = p1;
    let ((x2, y2), _) = p2;
    let x = i32::abs(x1 - x2);
    let y = i32::abs(y1 - y2);
    x + y
}

fn get_instructions_from_text(text: &str) -> Vec<String> {
    let instructions = text.split(',');
    instructions.map(|s| s.to_string()).collect()
}

fn parse_instruction(
    current_pos: (i32, i32, i32),
    instruction: &String,
) -> Result<(i32, i32, i32), Box<dyn Error>> {
    let (direction, value_digits) = instruction.split_at(1);
    let value: i32 = value_digits.parse()?;
    let delta = match direction {
        "U" => (0, value, value),
        "D" => (0, -value, value),
        "L" => (-value, 0, value),
        "R" => (value, 0, value),
        _ => return Err(From::from("Direction given not recognised")),
    };
    Ok((
        current_pos.0 + delta.0,
        current_pos.1 + delta.1,
        current_pos.2 + delta.2,
    ))
}

fn get_intersections(
    v1: Vec<(i32, i32, i32)>,
    v2: Vec<(i32, i32, i32)>,
) -> HashMap<(i32, i32), i32> {
    let map1: HashMap<_, _> = v1.into_iter().map(|(x, y, s)| ((x, y), s)).collect();
    let map2: HashMap<_, _> = v2.into_iter().map(|(x, y, s)| ((x, y), s)).collect();

    map1.into_iter()
        .filter_map(|(k, v)| map2.get(&k).map(|v2| (k, v + v2)))
        .collect()
}

fn get_points_between(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    let (x1, y1, s1) = p1;
    let (x2, y2, _) = p2;
    if x1 == x2 {
        if y1 < y2 {
            (y1..=y2).map(|y| (x1, y, (y - y1).abs() + s1)).collect()
        } else {
            (y2..=y1).map(|y| (x1, y, (y1 - y).abs() + s1)).collect()
        }
    } else {
        if x1 < x2 {
            (x1..=x2).map(|x| (x, y1, (x - x1).abs() + s1)).collect()
        } else {
            (x2..=x1).map(|x| (x, y1, (x1 - x).abs() + s1)).collect()
        }
    }
}

fn interpolate_vector(vector: &Vec<(i32, i32, i32)>) -> Vec<(i32, i32, i32)> {
    let mut interpolated_vec: Vec<(i32, i32, i32)> = Vec::new();
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
            let mut pos = (0, 0, 0);
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
        .map(|i| calc_manhattan_dist((0, 0, 0), i))
        .collect();
    let min_dist = distances.iter().min().unwrap();
    let min_steps = intersections.iter().min_by_key(|i| i.1).unwrap().1;
    println!("Part 01, {min_dist:?}");
    println!("Part 02: {min_steps:?}")

}
