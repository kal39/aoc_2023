use regex::Regex;

fn is_star(schematic: &String, row: i32, col: i32) -> bool {
    match schematic.split("\n").nth(row as usize) {
        Some(line) => match line.chars().nth(col as usize) {
            Some(c) => c == '*',
            None => false,
        },
        None => false,
    }
}

fn main() {
    let re = Regex::new(r"(\d+)").unwrap();
    let input = std::fs::read_to_string("input/day_03.txt").unwrap();
    let mut res = vec![(0, 1); input.len()];

    input.lines().enumerate().for_each(|(i, line)| {
        re.find_iter(line).for_each(|c| {
            let start = c.start() as i32;
            let len = (c.end() - c.start()) as i32;

            for row in (i as i32 - 1)..(i as i32 + 2) {
                for col in (start - 1)..(start + len + 1) {
                    if is_star(&input, row, col) {
                        let val = c.as_str().parse::<i32>().unwrap();
                        let idx = row as usize * line.len() + col as usize;
                        res[idx] = (res[idx].0 + 1, res[idx].1 * val);
                    }
                }
            }
        })
    });

    let res = res
        .iter()
        .filter(|(count, _)| *count == 2)
        .map(|(_, val)| val)
        .sum::<i32>();

    println!("result: {}", res);
}
