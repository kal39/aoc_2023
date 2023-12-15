fn hash(str: &str) -> u64 {
    let mut res = 0;
    str.chars().for_each(|c| {
        let c = c as u64;
        res += c;
        res *= 17;
        res %= 256;
    });
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
