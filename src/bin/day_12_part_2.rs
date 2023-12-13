use std::collections::HashMap;

fn can_be_broken(conditions: &[char], start: usize, len: usize) -> bool {
    if start + len > conditions.len() {
        return false;
    }

    if start > 0 && conditions[start - 1] == '#' {
        return false;
    }

    if start + len < conditions.len() && conditions[start + len] == '#' {
        return false;
    }

    for i in start..(start + len) {
        if conditions[i as usize] == '.' {
            return false;
        }
    }

    return true;
}

fn count_arrangements<'a>(
    conditions: &'a [char],
    groups: &'a [usize],
    dict: &mut HashMap<(&'a [char], &'a [usize]), u64>,
) -> u64 {
    if dict.contains_key(&(conditions, groups)) {
        return *dict.get(&(conditions, groups)).unwrap();
    }

    if groups.len() == 0 {
        return !(conditions.iter().any(|c| *c == '#')) as u64;
    }

    let mut arrangements = 0;

    for i in 0..conditions.len() {
        if conditions[..i].iter().any(|c| *c == '#') {
            continue;
        }

        if can_be_broken(conditions, i, groups[0]) {
            let conditions = &conditions[usize::min(i + groups[0] + 1, conditions.len())..];
            let groups = &groups[1..];
            let res = count_arrangements(conditions, groups, dict);
            dict.insert((conditions, groups), res);
            arrangements += res;
        }
    }

    arrangements
}

fn main() {
    let res = std::fs::read_to_string("input/day_12.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let mut conditions = l.split(" ").collect::<Vec<&str>>()[0]
                .chars()
                .collect::<Vec<char>>();

            let mut groups = l.split(" ").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            conditions.push('?');
            conditions = conditions.repeat(5)[..(conditions.len() * 5 - 1)].to_vec();

            groups = groups.repeat(5);

            count_arrangements(&conditions, &groups, &mut HashMap::new())
        })
        .sum::<u64>();

    println!("result: {}", res)
}
