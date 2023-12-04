fn main() {
    let res = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let winning = line
                .split("|")
                .nth(0)
                .unwrap()
                .split(":")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let matches = line
                .split("|")
                .nth(1)
                .unwrap()
                .split_whitespace()
                .map(|s| {
                    let val = s.parse::<u32>().unwrap();
                    winning.contains(&val) as u32
                })
                .sum::<u32>();

            match matches {
                0 => 0,
                n => (2 as u32).pow(n - 1),
            }
        })
        .sum::<u32>();

    println!("result: {}", res);
}
