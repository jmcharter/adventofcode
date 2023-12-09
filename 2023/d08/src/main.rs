use std::{
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

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    0
}

// fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
//     0
// }

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_process_part_one_input1() {
        let input_bytes = INPUT1.as_bytes();
        assert_eq!(2, process_part_one(BufReader::new(input_bytes)));
    }
    #[test]
    fn test_process_part_one_input2() {
        let input_bytes = INPUT2.as_bytes();
        assert_eq!(6, process_part_one(BufReader::new(input_bytes)));
    }

    // #[test]
    // fn test_process_part_two() {
    //     let input_bytes = INPUT.as_bytes();
    //     assert_eq!(71503, process_part_two(BufReader::new(input_bytes)));
    // }
}
