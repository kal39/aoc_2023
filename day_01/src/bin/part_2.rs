fn main() {
    let out = std::fs::read_to_string("input.txt")
        .expect("failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.replace("one", "o1e"))
        .map(|s| s.replace("two", "t2o"))
        .map(|s| s.replace("three", "t3e"))
        .map(|s| s.replace("four", "4"))
        .map(|s| s.replace("five", "5e"))
        .map(|s| s.replace("six", "6"))
        .map(|s| s.replace("seven", "7n"))
        .map(|s| s.replace("eight", "e8t"))
        .map(|s| s.replace("nine", "n9e"))
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|s| (s[0], s[s.len() - 1]))
        .map(|s| (s.0.to_digit(10).unwrap(), s.1.to_digit(10).unwrap()))
        .map(|s| 10 * s.0 + s.1)
        .sum::<u32>()
        .to_string();

    println!("result: {}", out);
}
