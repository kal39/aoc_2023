fn main() {
    let out = std::fs::read_to_string("input/day_01.txt")
        .expect("failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let numbers = s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let first = numbers.first().unwrap().to_digit(10).unwrap();
            let last = numbers.last().unwrap().to_digit(10).unwrap();
            10 * first + last
        })
        .sum::<u32>();

    println!("result: {}", out);
}
