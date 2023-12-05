use std::{
    env, fs,
    io::{self, BufReader, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file1 = fs::File::open(filename)?;
    let file2 = fs::File::open(filename)?;
    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    println!("Part one: {}", process_part_one(&mut reader1));
    println!("Part two: {}", process_part_two(&mut reader2));
    Ok(())
}

#[derive(Debug)]
struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    fn map_to_lines(&self, key: u32) -> u32 {
        for line in &self.lines {
            if line.in_range(key) {
                return line.map(key);
            }
        }
        key
    }
}

#[derive(Debug)]
struct MapLine {
    dest_range: u32,
    source_range: u32,
    range_length: u32,
}

impl MapLine {
    fn map(&self, key: u32) -> u32 {
        let diff = key - self.source_range;
        if self.dest_range as i64 + diff as i64 > 0 {
            return (self.dest_range as i64 + diff as i64) as u32;
        }
        key
    }

    fn in_range(&self, key: u32) -> bool {
        self.source_range <= key
            && (key as i64) < self.source_range as i64 + self.range_length as i64
    }
}

fn parse_input<R: Read>(reader: &mut BufReader<R>) -> (Vec<String>, Vec<Map>) {
    let mut almanac = String::new();
    reader
        .read_to_string(&mut almanac)
        .expect("read successful");
    let parts: Vec<&str> = almanac.split("\n\n").collect();
    let (seeds, others) = parts.split_first().expect("at least one part");
    let seeds: Vec<_> = seeds
        .split(": ")
        .last()
        .expect("at least one")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let maps: Vec<_> = others
        .iter()
        .map(|item| {
            let lines_iter = item
                .split(':')
                .last()
                .expect("exists")
                .trim()
                .split('\n')
                .map(|nums| {
                    let nums_split = nums.split_whitespace().collect::<Vec<_>>();
                    MapLine {
                        dest_range: nums_split[0].parse().expect("is digit"),
                        source_range: nums_split[1].parse().expect("is digit"),
                        range_length: nums_split[2].parse().expect("is digit"),
                    }
                });
            Map {
                lines: lines_iter.collect(),
            }
        })
        .collect();
    (seeds, maps)
}

fn process_part_one<R: Read>(reader: &mut BufReader<R>) -> u32 {
    let (seeds, maps) = parse_input(reader);
    let mut res = u32::MAX;
    for seed in &seeds {
        let mut val = seed.parse::<u32>().expect("is digits");
        for map in &maps {
            val = map.map_to_lines(val);
        }
        res = u32::min(res, val);
    }
    res
}

fn process_part_two<R: Read>(reader: &mut BufReader<R>) -> u32 {
    let (seeds, maps) = parse_input(reader);
    let seed_chunks: Vec<_> = seeds.chunks(2).collect();
    let mut res = u32::MAX;
    for chunk in seed_chunks {
        let range_start: u32 = chunk[0].parse().expect("is digits");
        let range_length: u32 = chunk[1].parse().expect("is digits");
        let range_end: u32 = range_start + range_length;
        for seed in range_start..range_end {
            let mut val = seed;
            for map in &maps {
                val = map.map_to_lines(val);
            }
            res = u32::min(res, val);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(35, process_part_one(&mut BufReader::new(input_bytes)));
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(46, process_part_two(&mut BufReader::new(input_bytes)));
    }
}
