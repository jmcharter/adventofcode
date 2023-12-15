use std::error::Error;
use utils::read_file_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err("Please provide a filename as an argument".into());
    }
    let filename = &args[1];
    let input = read_file_to_string(filename)?;
    let range = split_range(&input);
    println!("{range:?}");
    Ok(())
}

fn split_range(range: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = range.trim().split("-").collect();
    if parts.len() != 2 {
        None
    } else {
        if let (Ok(part1), Ok(part2)) = (parts[0].parse(), parts[1].parse()) {
            Some((part1, part2))
        } else {
            None
        }
    }
}
