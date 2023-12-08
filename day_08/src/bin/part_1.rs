use std::collections::HashMap;

use regex::Regex;

fn main() {
    let re = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();

    let actions = input.lines().nth(0).unwrap().chars().collect::<Vec<char>>();
    let nodes = input
        .lines()
        .skip(2)
        .map(|l| {
            let capture: [&str; 3] = re.captures(l).unwrap().extract().1;
            (capture[0], (capture[1], capture[2]))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    let mut curr = "AAA";
    let mut steps = 0;

    while curr != "ZZZ" {
        for action in actions.iter() {
            curr = match action {
                'L' => nodes.get(curr).unwrap().0,
                'R' => nodes.get(curr).unwrap().1,
                _ => panic!(),
            };
            steps += 1
        }
    }

    println!("result: {}", steps);
}
