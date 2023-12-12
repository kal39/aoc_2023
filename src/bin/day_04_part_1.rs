use regex::Regex;

fn main() {
    let re = Regex::new(r"Card *\d+: (.*) \| (.*)").unwrap();

    let res = std::fs::read_to_string("input/day_04.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let [winning, available] = re.captures(line).unwrap().extract().1;

            let winning = winning
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let matches = available
                .split_whitespace()
                .map(|s| winning.contains(&s.parse::<u32>().unwrap()) as u32)
                .sum::<u32>();

            match matches {
                0 => 0,
                n => (2 as u32).pow(n - 1),
            }
        })
        .sum::<u32>();

    println!("result: {}", res);
}
