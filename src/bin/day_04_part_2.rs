use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("input/day_04.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let re = Regex::new(r"Card *\d+: (.*) \| (.*)").unwrap();
    let mut copies = vec![1; input.len()];

    input.iter().enumerate().for_each(|(i, line)| {
        let [winning, available] = re.captures(line).unwrap().extract().1;

        let winning = winning
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let matches = available
            .split_whitespace()
            .map(|s| winning.contains(&s.parse::<u32>().unwrap()) as u32)
            .sum::<u32>();

        for j in (i + 1)..(i + matches as usize + 1) {
            copies[j] = copies[j] + 1 * copies[i];
        }
    });

    let res = copies.iter().sum::<u32>();
    println!("result: {}", res);
}
