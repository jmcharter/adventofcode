use std::{
    env, fs,
    io::{self, BufRead, BufReader},
};

#[derive(Debug)]
struct Sample {
    r: u32,
    g: u32,
    b: u32,
}

fn split_cube_set(set: &[&str], colour: &str) -> Option<u32> {
    match set.iter().find(|x| x.ends_with(colour)) {
        Some(item) => item
            .trim()
            .split(' ')
            .next()
            .expect("Found item is present")
            .parse::<u32>()
            .ok(),
        None => None,
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = fs::File::open(filename)?;
    let reader = BufReader::new(file);
    let mut valid_game_ids_sum = 0;
    let mut game_power_sum = 0;
    let max_r = 12;
    let max_g = 13;
    let max_b = 14;
    for line_result in reader.lines() {
        let mut valid_game = true;
        let line = line_result.unwrap();
        let line_split: Vec<_> = line.split(':').collect();
        let game_id = line_split[0]
            .split(' ')
            .collect::<Vec<_>>()
            .last()
            .expect("item exists")
            .parse::<u32>()
            .expect("is a number");
        let rest = line_split[1];
        let cube_sets = rest.split(';');
        let samples: Vec<Sample> = cube_sets
            .map(|set| {
                let set_split: Vec<_> = set.split(',').collect();
                let r = split_cube_set(&set_split, "red").unwrap_or(0);
                let g = split_cube_set(&set_split, "green").unwrap_or(0);
                let b = split_cube_set(&set_split, "blue").unwrap_or(0);
                Sample { r, g, b }
            })
            .collect();
        let mut highest_r = 0;
        let mut highest_g = 0;
        let mut highest_b = 0;
        for sample in &samples {
            if !(sample.r <= max_r && sample.g <= max_g && sample.b <= max_b) {
                valid_game = false;
            }
            highest_r = u32::max(highest_r, sample.r);
            highest_g = u32::max(highest_g, sample.g);
            highest_b = u32::max(highest_b, sample.b);
        }
        if valid_game {
            valid_game_ids_sum += game_id;
        }
        game_power_sum += highest_r * highest_g * highest_b;
    }
    println!("Sum of game ids: {valid_game_ids_sum}");
    println!("Sum of game powers: {game_power_sum}");
    Ok(())
}
