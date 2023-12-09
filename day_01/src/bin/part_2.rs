fn main() {
    let out = std::fs::read_to_string("input.txt")
        .expect("failed to read input")
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| {
            let s = s
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            let numbers = s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let first = numbers.first().unwrap().to_digit(10).unwrap();
            let last = numbers.last().unwrap().to_digit(10).unwrap();
            10 * first + last
        })
        .sum::<u32>();

    println!("result: {}", out);
}
