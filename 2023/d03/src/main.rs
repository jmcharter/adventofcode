use std::{
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

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let mut lines = reader.lines().peekable();
    let mut prev_line: Option<String> = None;
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let current_line = line.expect("line exists");
        let next_line = match lines.peek() {
            Some(Ok(line)) => Some(line),
            Some(Err(_)) => None,
            None => None,
        };
        match (prev_line, next_line) {
            (None, Some(next)) => {
                let lines = vec![&current_line, next];
                sum += parse_lines(lines, true);
            }
            (Some(prev), Some(next)) => {
                let lines = vec![&prev, &current_line, next];
                sum += parse_lines(lines, false);
            }
            (Some(prev), None) => {
                let lines = vec![&prev, &current_line];
                sum += parse_lines(lines, false);
            }
            (None, None) => {}
        }

        prev_line = Some(current_line);
    }
    sum
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
    let mut lines = reader.lines().peekable();
    let mut prev_line: Option<String> = None;
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let current_line = line.expect("line exists");
        let next_line = match lines.peek() {
            Some(Ok(line)) => Some(line),
            Some(Err(_)) => None,
            None => None,
        };
        match (prev_line, next_line) {
            (None, Some(next)) => {
                let lines = vec![&current_line, next];
                sum += parse_lines_for_gears(lines, true);
            }
            (Some(prev), Some(next)) => {
                let lines = vec![&prev, &current_line, next];
                sum += parse_lines_for_gears(lines, false);
            }
            (Some(prev), None) => {
                let lines = vec![&prev, &current_line];
                sum += parse_lines_for_gears(lines, false);
            }
            (None, None) => {}
        }

        prev_line = Some(current_line);
    }

    sum
}

fn parse_lines(lines: Vec<&String>, first_line: bool) -> u32 {
    let mut sum = 0;
    let mut num = 0;
    let mut valid = false;
    let mut char_vec: Vec<Vec<char>> = Vec::new();
    for line in lines {
        char_vec.push(line.chars().collect());
    }
    let chars = match first_line {
        true => &char_vec[0],
        false => &char_vec[1],
    };
    for i in 0..chars.len() {
        if chars[i].is_digit(10) {
            // Add the digit to the number
            num = num * 10 + chars[i].to_digit(10).expect("is digit");

            // Check the surrounding character for non-period symbols
            for &x in &[-1, 0, 1] {
                for chars in &char_vec {
                    if (i as isize + x).is_positive() && ((i as isize + x) as usize) < chars.len() {
                        let index = (i as isize + x) as usize;
                        if !chars[index].is_digit(10) && chars[index] != '.' {
                            valid = true;
                        }
                    }
                }
            }
        } else {
            if valid {
                sum += num;
            }
            valid = false;
            num = 0;
        }
    }
    if valid {
        sum += num;
    }
    sum
}

fn parse_lines_for_gears(lines: Vec<&String>, first_line: bool) -> u32 {
    let mut sum = 0;
    let mut char_vec: Vec<Vec<char>> = Vec::new();
    for line in &lines {
        char_vec.push(line.chars().collect());
    }
    let chars = match first_line {
        true => &char_vec[0],
        false => &char_vec[1],
    };
    for i in 0..chars.len() {
        if chars[i] == '*' {
            let surrounding_nums = get_surrounding_numbers(&lines, i);
            let product = match surrounding_nums.len() {
                0 | 1 => 0,
                _ => surrounding_nums.iter().product(),
            };
            sum += product;
        }
    }
    sum
}

fn get_surrounding_numbers(lines: &Vec<&String>, gear_pos: usize) -> Vec<u32> {
    let mut nums: Vec<u32> = Vec::new();
    let mut num: u32 = 0;
    let mut valid = false;
    for line in lines {
        for (i, char) in line.chars().enumerate() {
            if char.is_digit(10) {
                num = num * 10 + char.to_digit(10).expect("is digit");
                if [gear_pos - 1, gear_pos, gear_pos + 1].contains(&i) {
                    valid = true;
                }
            } else if num > 0 && valid {
                nums.push(num);
                num = 0;
                valid = false;
            } else {
                num = 0;
                valid = false;
            }
        }
        if num > 0 && valid {
            nums.push(num);
        }
        num = 0;
        valid = false;
    }
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(4361, process_part_one(BufReader::new(input_bytes)));
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(467835, process_part_two(BufReader::new(input_bytes)));
    }
}
