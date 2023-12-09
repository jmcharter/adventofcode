use std::{
    collections::HashMap,
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

#[derive(Debug)]
struct Node {
    value: String,
    left: Option<String>,
    right: Option<String>,
}

impl Node {
    fn new(value: String) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}

fn parse_data<R: Read>(mut reader: BufReader<R>) -> (String, HashMap<String, Node>) {
    let directions = reader
        .by_ref()
        .lines()
        .next()
        .unwrap_or(Ok("".to_string()))
        .expect("at least one line");
    reader.by_ref().lines().next();
    let mut data = Vec::new();
    for line in reader.lines() {
        let line = line.expect("line found");
        data.push(line);
    }
    let mut nodes: HashMap<String, Node> = HashMap::new();
    for line in &data {
        let parts: Vec<&str> = line.split('=').collect();
        let value = parts[0].trim();
        let node = Node::new(value.to_string());
        nodes.insert(value.to_string(), node);
    }
    for line in &data {
        let parts: Vec<&str> = line
            .split(['=', '(', ')', ','])
            .filter(|s| !s.trim().is_empty())
            .collect();
        let value = parts[0].trim();
        let left = parts[1].trim();
        let right = parts[2].trim();
        if let Some(node) = nodes.get_mut(value) {
            node.left = Some(left.to_string());
            node.right = Some(right.to_string());
        }
    }
    (directions, nodes)
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let (directions, nodes) = parse_data(reader);
    let start_node = nodes.get("AAA").expect("Node 'AAA' exists");
    let steps = calc_steps(&directions, start_node, &nodes);
    steps
}

fn calc_steps(directions: &str, start_node: &Node, nodes: &HashMap<String, Node>) -> u32 {
    let mut steps = 0;
    let mut curr_node = start_node;
    for dir in directions.chars().cycle() {
        if curr_node.value.ends_with('Z') {
            break;
        };
        steps += 1;
        curr_node = match dir {
            'L' => {
                if let Some(left_key) = &curr_node.left {
                    nodes.get(left_key).unwrap_or(curr_node)
                } else {
                    curr_node
                }
            }
            'R' => {
                if let Some(right_key) = &curr_node.right {
                    nodes.get(right_key).unwrap_or(curr_node)
                } else {
                    curr_node
                }
            }
            _ => curr_node,
        }
    }

    steps
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn process_part_two<R: Read>(reader: BufReader<R>) -> u64 {
    let (directions, nodes) = parse_data(reader);
    let start_nodes: HashMap<_, _> = nodes.iter().filter(|(k, _)| k.ends_with('A')).collect();
    let step_counts: u64 = start_nodes
        .iter()
        .map(|(_, node)| calc_steps(&directions, node, &nodes))
        .fold(1, |acc, num| lcm(acc, num as u64));
    step_counts
}

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
ZZZ = (ZZZ, ZZZ)";

    const INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
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

    #[test]
    fn test_process_part_two() {
        let input_bytes = INPUT3.as_bytes();
        assert_eq!(6, process_part_two(BufReader::new(input_bytes)));
    }
}
