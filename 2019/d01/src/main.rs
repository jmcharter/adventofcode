fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    let p1 = p1(&input);
    let p2 = p2(&input);
    println!("P1: {p1}");
}

fn p1(input: &str) -> i32 {
    let lines = input.lines();
    lines.fold(0, |acc, e| {
        let val = (e.parse::<i32>().unwrap() / 3) - 2;
        acc + val
    })
}

fn p2(input: &str) {
    let lines = input.lines();
}
