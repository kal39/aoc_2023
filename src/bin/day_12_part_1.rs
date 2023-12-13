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

fn count_arrangements(conditions: &[char], groups: &[usize], level: usize) -> i32 {
    if groups.len() == 0 {
        return !(conditions.iter().any(|c| *c == '#')) as i32;
    }

    let mut arrangements = 0;

    for i in 0..conditions.len() {
        if conditions[..i].iter().any(|c| *c == '#') {
            continue;
        }

        if can_be_broken(conditions, i, groups[0]) {
            let conditions = &conditions[usize::min(i + groups[0] + 1, conditions.len())..];
            let groups = &groups[1..];
            arrangements += count_arrangements(conditions, groups, level + 1);
        }
    }

    arrangements
}

fn main() {
    let res = std::fs::read_to_string("input/day_12.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let conditions = l.split(" ").collect::<Vec<&str>>()[0]
                .chars()
                .collect::<Vec<char>>();
            let groups = l.split(" ").collect::<Vec<&str>>()[1]
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            count_arrangements(&conditions, &groups, 0)
        })
        .sum::<i32>();

    println!("result: {}", res)
}
