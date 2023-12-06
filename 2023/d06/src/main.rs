use std::{
    collections::HashSet,
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

fn parse_data<R: Read>(reader: BufReader<R>) -> Vec<Vec<u64>> {
    let lines = reader.lines().flatten();
    let data: Vec<_> = lines
        .map(|line| {
            line.split(':')
                .last()
                .expect("text after colon")
                .split_whitespace()
                .map(|s| s.parse::<u64>().expect("numbers"))
                .collect::<Vec<_>>()
        })
        .collect();
    data
}

fn calculate_ways_to_win(time: u64, dist: u64) -> HashSet<u64> {
    let mut wins = HashSet::<u64>::new();
    for t in 1..time {
        let d = t * (time - t);
        if d > dist {
            wins.insert(t);
        }
    }
    wins
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u64 {
    let data = parse_data(reader);
    let results: Vec<_> = data[0].iter().zip(data[1].iter()).collect();
    let mut win_method_qty: Vec<u64> = Vec::new();
    for r in results {
        win_method_qty.push(calculate_ways_to_win(*r.0, *r.1).len() as u64);
    }
    win_method_qty.iter().product()
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u64 {
    let data = parse_data(reader);
    let joined_data: Vec<_> = data
        .iter()
        .map(|v| {
            v.iter()
                .map(|d| d.to_string())
                .collect::<Vec<_>>()
                .join("")
                .parse::<u64>()
                .expect("all digits")
        })
        .collect();

    calculate_ways_to_win(joined_data[0], joined_data[1]).len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(288, process_part_one(BufReader::new(input_bytes)));
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(71503, process_part_two(BufReader::new(input_bytes)));
    }
}
