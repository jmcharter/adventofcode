fn main() {
    let input = std::fs::read_to_string("src/input").unwrap();
    let p1 = p1(&input);
    let p2 = p2(&input);
    println!("P1: {p1}");
    println!("P2: {p2}");
}

fn p1(input: &str) -> i32 {
    let lines = input.lines();
    lines.fold(0, |acc, e| {
        let val = (e.parse::<i32>().unwrap() / 3) - 2;
        acc + val
    })
}

fn p2(input: &str) -> u32 {
    let lines = input.lines();
    lines.fold(0, |acc, e| {
        let val = e.parse::<u32>().unwrap();
        let mut sub_req = get_fuel_req(val);
        let mut req = sub_req;
        while sub_req > 0 {
            sub_req = get_fuel_req(sub_req);
            req += sub_req;
        }
        acc + req
    })
}

fn get_fuel_req(mass: u32) -> u32 {
    let div_result = mass / 3;
    if div_result < 2 {
        0
    } else {
        div_result - 2
    }
}
