use std::{
    env, fs,
    io::{self, BufReader, Read},
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

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    288
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
