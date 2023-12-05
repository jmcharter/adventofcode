use std::{
    collections::BTreeMap,
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

#[derive(Debug)]
struct CardInfo {
    count: u32,
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
    let mut cards: BTreeMap<u32, CardInfo> = BTreeMap::new();
    for line in reader.lines().flatten() {
        let card_data: Vec<_> = line.split(": ").collect();
        let card_id: u32 = card_data[0]
            .replace("Card", "")
            .trim()
            .parse()
            .expect("is digit");
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
        let card_details = CardInfo { count: 1 };
        if let Some(old_card_info) = cards.insert(card_id, card_details) {
            let card_entry = cards.get_mut(&card_id);
            card_entry.expect("card exists").count += old_card_info.count;
        };
        let current_card = cards.get(&card_id).expect("card exists");
        if matches > 0 {
            for _ in 0..current_card.count {
                for i in (card_id + 1)..=(matches as u32) + card_id {
                    let new_card_info = CardInfo { count: 1 };
                    if let Some(old_card_info) = cards.insert(i, new_card_info) {
                        let card_entry = cards.get_mut(&i).expect("card exists");
                        card_entry.count += old_card_info.count;
                    }
                }
            }
        }
    }
    let sum = cards.iter().fold(0, |acc, c| acc + c.1.count);
    sum
}

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

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(30, process_part_two(BufReader::new(input_bytes)));
    }
}
