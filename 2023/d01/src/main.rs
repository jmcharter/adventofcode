use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);

    let number_map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);

    let mut total = 0;
    for _line in reader.lines() {
        let digits = get_text_numbers(_line.unwrap(), &number_map);
        if !digits.is_empty() {
            let digit_first = digits.first().unwrap();
            let digit_last = digits.last().unwrap();
            let mut cat = String::new();
            cat.push(*digit_first);
            cat.push(*digit_last);
            let cat: i32 = cat.parse().unwrap();
            total += cat;
        }
    }
    println!("{total}");
    Ok(())
}

fn get_text_numbers(text: String, number_map: &HashMap<&str, &str>) -> Vec<char> {
    let mut digits: Vec<char> = Vec::new();
    if text.is_empty() {
        return digits;
    }
    let mut sample = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut ptr1: usize = 0;
    let mut ptr2: usize;
    while ptr1 < chars.len() {
        sample.clear();
        ptr2 = ptr1 + 1;
        if chars[ptr1].is_digit(10) {
            digits.push(chars[ptr1]);
            sample.clear();
            ptr1 += 1;
            continue;
        }
        sample.push(chars[ptr1]);
        while ptr2 < chars.len() {
            if chars[ptr2].is_digit(10) {
                sample.clear();
                break;
            }
            sample.push(chars[ptr2]);
            if number_map.contains_key(&sample.as_str()) {
                let str_digit: char = number_map.get(&sample.as_str()).unwrap().parse().unwrap();
                digits.push(str_digit);
                sample.clear();
                break;
            }
            ptr2 += 1;
        }
        ptr1 += 1;
    }

    digits
}
