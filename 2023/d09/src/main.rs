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
    // let reader2 = BufReader::new(file2);

    println!("Part one: {}", process_part_one(reader1));
    // println!("Part two: {}", process_part_two(reader2));
    Ok(())
}

fn parse_data<R: Read>(reader: BufReader<R>) -> Vec<String> {
    let lines = reader.lines();
    lines.flatten().collect::<Vec<_>>()
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u64 {
    0
}

// fn process_part_two<R: Read>(reader: BufReader<R>) -> u64 {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(0, process_part_one(BufReader::new(input_bytes)));
    }

    // #[test]
    // fn test_process_part_two() {
    //     let input_bytes = INPUT.as_bytes();
    //     assert_eq!(71503, process_part_two(BufReader::new(input_bytes)));
    // }
}
