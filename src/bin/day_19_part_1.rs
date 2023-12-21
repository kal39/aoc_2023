use std::collections::HashMap;

use regex::Regex;

fn main() {
    let re_workflows = Regex::new(r"(\w*)\{(.*),(.*)\}").unwrap();
    let re_workflow = Regex::new(r"(\w)([<>])(\d*):(\w*)").unwrap();
    let re_part = Regex::new(r"\{x=(\d*),m=(\d*),a=(\d*),s=(\d*)\}").unwrap();

    let input = std::fs::read_to_string("input/day_19.txt").unwrap();

    let workflows = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .map(|line| {
            let captures = re_workflows.captures(line).unwrap();
            let name = captures.get(1).unwrap().as_str();
            let default = captures.get(3).unwrap().as_str();
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
                        captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        captures.get(4).unwrap().as_str(),
                    )
                })
                .collect::<Vec<_>>();

            (name, (default, rules))
        })
        .collect::<HashMap<_, _>>();

    let res = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .map(|line| {
            let captures = re_part.captures(line).unwrap();
            let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let m = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let a = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let s = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

            let mut workflow = "in";
            while !(workflow == "A" || workflow == "R") {
                let (default, rules) = workflows.get(workflow).unwrap();
                let accept = rules
                    .iter()
                    .map(|(category, comparator, comp_val, dest)| {
                        let val = match category {
                            'x' => x,
                            'm' => m,
                            'a' => a,
                            's' => s,
                            _ => panic!(),
                        };

                        if *comparator == '<' && val < *comp_val {
                            dest
                        } else if *comparator == '>' && val > *comp_val {
                            dest
                        } else {
                            ""
                        }
                    })
                    .filter(|dest| *dest != "")
                    .next();
                match accept {
                    Some(dest) => workflow = dest,
                    None => workflow = default,
                }
            }

            if workflow == "A" {
                x + m + a + s
            } else {
                0
            }
        })
        .sum::<i32>();

    println!("res: {}", res);
}
