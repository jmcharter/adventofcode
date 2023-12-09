use std::{
    cell::RefCell,
    collections::HashMap,
    env, fs,
    io::{self, BufRead, BufReader, Read},
    rc::Rc,
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

#[derive(Debug)]
struct Node {
    value: String,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            left: None,
            right: None,
        }))
    }
}

fn parse_data<R: Read>(mut reader: BufReader<R>) -> (String, Rc<RefCell<Node>>) {
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
    let mut nodes: HashMap<String, Rc<RefCell<Node>>> = HashMap::new();
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
        if let Some(node_rc) = nodes.get(value) {
            let mut node_borrowed = node_rc.borrow_mut();
            if value != left {
                node_borrowed.left = nodes.get(left).cloned();
            }
            if value != right {
                node_borrowed.right = nodes.get(right).cloned();
            }
        }
    }
    let start_node = nodes.get("AAA").expect("Node 'AAA' exists").clone();
    (directions, start_node)
}

fn process_part_one<R: Read>(reader: BufReader<R>) -> u32 {
    let (directions, node) = parse_data(reader);
    let mut steps = 0;
    let mut curr_node = node;
    for dir in directions.chars().cycle() {
        if curr_node.borrow().value == "ZZZ" {
            break;
        }
        curr_node = match dir {
            'L' => {
                steps += 1;
                curr_node.borrow().left.as_ref().unwrap().clone()
            }
            'R' => {
                steps += 1;
                curr_node.borrow().right.as_ref().unwrap().clone()
            }
            _ => curr_node,
        };
    }
    steps
}

// fn process_part_two<R: Read>(reader: BufReader<R>) -> u32 {
// 0
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
