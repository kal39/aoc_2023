use regex::Regex;

fn is_symbol(schematic: &String, row: i32, col: i32) -> bool {
    match schematic.split("\n").nth(row as usize) {
        Some(line) => match line.chars().nth(col as usize) {
            Some(c) => !c.is_ascii_digit() && c != '.',
            None => false,
        },
        None => false,
    }
}

fn main() {
    let re = Regex::new(r"(\d+)").unwrap();
    let input = std::fs::read_to_string("input/day_03.txt").unwrap();

    let res = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            re.find_iter(line)
                .map(|c| {
                    let start = c.start() as i32;
                    let len = (c.end() - c.start()) as i32;

                    let mut adjacent = false;
                    for row in (i as i32 - 1)..(i as i32 + 2) {
                        for col in (start - 1)..(start + len + 1) {
                            if is_symbol(&input, row, col) {
                                adjacent = true;
                            }
                        }
                    }

                    c.as_str().parse::<i32>().unwrap() * adjacent as i32
                })
                .sum::<i32>()
        })
        .sum::<i32>();

    println!("result: {}", res);
}
