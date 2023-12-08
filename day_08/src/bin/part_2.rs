use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;

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

    let steps = nodes
        .iter()
        .filter(|(n, _)| n.chars().last().unwrap() == 'A')
        .map(|(&n, _)| {
            let mut curr = n;
            let mut steps: u64 = 0;

            while curr.chars().last().unwrap() != 'Z' {
                for action in actions.iter() {
                    curr = match action {
                        'L' => nodes.get(curr).unwrap().0,
                        'R' => nodes.get(curr).unwrap().1,
                        _ => panic!(),
                    };
                    steps += 1
                }
            }

            steps
        })
        .reduce(lcm)
        .unwrap();

    println!("result: {:?}", steps);
}
