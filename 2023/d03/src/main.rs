use std::{
    env, fs,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    println!("Part one: {}", process_part_one(reader));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_one() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let input_bytes = input.as_bytes();
        assert_eq!(4361, process_part_one(BufReader::new(input_bytes)));
    }
}
