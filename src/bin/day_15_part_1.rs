fn hash(s: &str) -> u64 {
    let mut res = 0;
    s.chars().for_each(|c| res = ((res + c as u64) * 17) % 256);
    res
}

fn main() {
    let res = std::fs::read_to_string("input/day_15.txt")
        .unwrap()
        .trim()
        .split(",")
        .map(hash)
        .sum::<u64>();

    println!("result: {}", res);
}
