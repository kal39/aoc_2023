use std::collections::HashMap;

use regex::Regex;

fn count_combinations(
    workflows: &HashMap<String, (String, Vec<(char, char, i128, String)>)>,
    workflow: &str,
    mut state: [(i128, i128); 4],
) -> i128 {
    match workflow {
        "A" => state
            .iter()
            .map(|(min, max)| i128::abs(max - min) + 1)
            .product::<i128>(),
        "R" => 0,
        _ => {
            let workflow = workflows.get(workflow).unwrap();
            let mut total = 0;

            for (category, comparator, comp_val, dest) in workflow.1.iter() {
                let idx = match category {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => panic!(),
                };

                let new_state = match comparator {
                    '<' => {
                        let mut new_state = state.clone();
                        new_state[idx].1 = i128::min(*comp_val - 1, state[idx].1);
                        new_state
                    }
                    '>' => {
                        let mut new_state = state.clone();
                        new_state[idx].0 = i128::max(*comp_val + 1, state[idx].0);
                        new_state
                    }
                    _ => panic!(),
                };

                total += count_combinations(workflows, dest, new_state);

                state = match comparator {
                    '<' => {
                        let mut new_state = state.clone();
                        new_state[idx].0 = i128::max(*comp_val, state[idx].0);
                        new_state
                    }
                    '>' => {
                        let mut new_state = state.clone();
                        new_state[idx].1 = i128::min(*comp_val, state[idx].1);
                        new_state
                    }
                    _ => panic!(),
                };
            }

            total + count_combinations(workflows, &workflow.0, state)
        }
    }
}

fn main() {
    let re_workflows = Regex::new(r"(\w*)\{(.*),(.*)\}").unwrap();
    let re_workflow = Regex::new(r"(\w)([<>])(\d*):(\w*)").unwrap();

    let workflows = std::fs::read_to_string("input/day_19.txt")
        .unwrap()
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| {
            let captures = re_workflows.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str().to_string();
            let default = captures.get(3).unwrap().as_str().to_string();
            let rules = captures
                .get(2)
                .unwrap()
                .as_str()
                .split(",")
                .map(|rule| {
                    let captures = re_workflow.captures(rule).unwrap();
                    (
                        captures.get(1).unwrap().as_str().chars().next().unwrap(),
                        captures.get(2).unwrap().as_str().chars().next().unwrap(),
                        captures.get(3).unwrap().as_str().parse::<i128>().unwrap(),
                        captures.get(4).unwrap().as_str().to_string(),
                    )
                })
                .collect::<Vec<_>>();

            (name, (default, rules))
        })
        .collect::<HashMap<_, _>>();

    let res = count_combinations(
        &workflows,
        "in",
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
    );

    println!("result: {}", res);
}
