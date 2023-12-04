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
    // println!("Part two: {}", process_part_two(reader2));
    Ok(())
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let mut sum = 0;
    for line in reader.lines().flatten() {
        let card_data: Vec<_> = line.split(": ").collect();
        let all_numbers = card_data[1];
        let number_parts: Vec<Vec<String>> = all_numbers
            .split('|')
            .map(|x| {
                x.replace("  ", " ")
                    .split_whitespace()
                    .map(|val| val.to_string())
                    .collect()
            })
            .collect();
        let (winning_nums, owned_nums) = (&number_parts[0], &number_parts[1]);
        let matches = owned_nums
            .iter()
            .filter(|num| winning_nums.contains(num))
            .count();
        if matches > 0 {
            sum += 2_u32.pow((matches - 1) as u32);
        }
    }
    sum
}

// fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
//     1
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(13, process_part_one(BufReader::new(input_bytes)));
    }

    // #[test]
    // fn test_process_part_two() {
    //     let input_bytes = INPUT.as_bytes();
    //     assert_eq!(467835, process_part_two(BufReader::new(input_bytes)));
    // }
}
