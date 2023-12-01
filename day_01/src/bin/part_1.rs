fn main() {
    let out = std::fs::read_to_string("input.txt")
        .expect("failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|s| (s[0], s[s.len() - 1]))
        .map(|s| (s.0.to_digit(10).unwrap(), s.1.to_digit(10).unwrap()))
        .map(|s| 10 * s.0 + s.1)
        .sum::<u32>()
        .to_string();

    println!("result: {}", out);
}
