use std::{
    collections::HashSet,
    env, fs,
    io::{self, BufRead, BufReader, Read},
};

type Galaxies = Vec<(u8, u8)>;

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

fn parse_data<R: Read>(reader: BufReader<R>) -> Vec<String> {
    let lines = reader.lines();
    lines.flatten().collect::<Vec<_>>()
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u64 {
    let grid = parse_data(reader);
    let (galaxies, expanded_rows, expanded_cols) = get_galaxies_and_expansions(grid);
    get_all_galaxy_distances_sum(galaxies, expanded_rows, expanded_cols, 2)
}
fn process_part_two<R: Read>(reader: BufReader<R>) -> u64 {
    let grid = parse_data(reader);
    let (galaxies, expanded_rows, expanded_cols) = get_galaxies_and_expansions(grid);
    get_all_galaxy_distances_sum(galaxies, expanded_rows, expanded_cols, 1_000_000)
}

fn get_galaxies_and_expansions(grid: Vec<String>) -> (Galaxies, HashSet<usize>, HashSet<usize>) {
    let mut expanded_rows: HashSet<_> = (0..grid.len()).collect();
    let mut expanded_cols: HashSet<_> = (0..grid[0].len()).collect();
    let mut galaxies: Vec<(u8, u8)> = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, col) in row.chars().enumerate() {
            if col == '#' {
                galaxies.push((x as u8, y as u8));
                expanded_rows.remove(&y);
                expanded_cols.remove(&x);
            }
        }
    }
    (galaxies, expanded_cols, expanded_rows)
}

fn get_all_galaxy_distances_sum(
    galaxies: Vec<(u8, u8)>,
    expanded_rows: HashSet<usize>,
    expanded_cols: HashSet<usize>,
    expansion_factor: u64,
) -> u64 {
    let mut sum = 0;
    for g1 in 0..galaxies.len() {
        for g2 in (g1 + 1)..galaxies.len() {
            let (x1, y1) = galaxies[g1];
            let (x2, y2) = galaxies[g2];

            let dist_x: usize = (u8::min(x1, x2)..u8::max(x1, x2))
                .map(|x| {
                    if expanded_rows.contains(&(x as usize)) {
                        (expansion_factor) as usize
                    } else {
                        1
                    }
                })
                .sum();
            let dist_y: usize = (u8::min(y1, y2)..u8::max(y1, y2))
                .map(|y| {
                    if expanded_cols.contains(&(y as usize)) {
                        (expansion_factor) as usize
                    } else {
                        1
                    }
                })
                .sum();
            sum += dist_x as u64 + dist_y as u64;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_process_part_one() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(374, process_part_one(BufReader::new(input_bytes)));
    }

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT.as_bytes();
        assert_eq!(82000210, process_part_two(BufReader::new(input_bytes)));
    }
}
