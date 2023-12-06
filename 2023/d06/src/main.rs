use std::{
    collections::HashSet,
    env, fs,
    io::{self, BufRead, BufReader, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file1 = fs::File::open(filename)?;
    // let file2 = fs::File::open(filename)?;
    let reader1 = BufReader::new(file1);
    // let mut reader2 = BufReader::new(file2);

    println!("Part one: {}", process_part_one(reader1));
    // println!("Part two: {}", process_part_two(&mut reader2));
    Ok(())
}

fn calculate_ways_to_win(time: u32, dist: u32) -> HashSet<u32> {
    let mut wins = HashSet::<u32>::new();
    for t in 1..time {
        let d = t * (time - t);
        if d > dist {
            wins.insert(t);
        }
    }
    wins
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let lines = reader.lines().flatten();
    let data: Vec<_> = lines
        .map(|line| {
            line.split(':')
                .last()
                .expect("text after colon")
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("numbers"))
                .collect::<Vec<_>>()
        })
        .collect();
    let results: Vec<_> = data[0].iter().zip(data[1].iter()).collect();
    let mut win_method_qty: Vec<u32> = Vec::new();
    for r in results {
        win_method_qty.push(calculate_ways_to_win(*r.0, *r.1).len() as u32);
    }
    win_method_qty.iter().product()
}

// fn process_part_two<R: Read>(reader: &mut BufReader<R>) -> u32 {
//     0
// }

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

    // #[test]
    // fn test_process_part_two() {
    //     let input_bytes = INPUT.as_bytes();
    //     assert_eq!(46, process_part_two(&mut BufReader::new(input_bytes)));
    // }
}
