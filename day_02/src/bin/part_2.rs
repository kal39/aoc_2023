use regex::Regex;

fn p_draw(s: &str, re: &Regex) -> i32 {
    return match re.captures(s) {
        Some(c) => c.extract::<1>().1[0].parse::<i32>().unwrap(),
        None => 0,
    };
}

fn main() {
    let re_line = Regex::new(r"^Game (\d*): (.*)$").unwrap();
    let re_r = Regex::new(r"(\d+) red").unwrap();
    let re_g = Regex::new(r"(\d+) green").unwrap();
    let re_b = Regex::new(r"(\d+) blue").unwrap();

    let out = std::fs::read_to_string("input.txt")
        .expect("failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let captures = re_line.captures(&s).unwrap().extract::<2>().1;
            let draws = captures[1]
                .split(";")
                .map(|s| (p_draw(s, &re_r), p_draw(s, &re_g), p_draw(s, &re_b)))
                .fold((0, 0, 0), |max, cur| {
                    (
                        i32::max(max.0, cur.0),
                        i32::max(max.1, cur.1),
                        i32::max(max.2, cur.2),
                    )
                });
            draws
        })
        .map(|s| s.0 * s.1 * s.2)
        .sum::<i32>();

    print!("result: {}", out);
}
