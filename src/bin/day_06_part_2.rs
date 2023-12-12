fn main() {
    let input = std::fs::read_to_string("input/day_06.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let t = input[0]
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let d = input[1]
        .split(":")
        .nth(1)
        .unwrap()
        .replace(" ", "")
        .parse::<f64>()
        .unwrap();

    let lower_bound = (t - f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0;
    let upper_bound = (t + f64::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0;

    let lower_bound = lower_bound.floor() as i32 + 1;
    let upper_bound = upper_bound.ceil() as i32 - 1;

    let res = upper_bound - lower_bound + 1;

    print!("result: {}", res);
}
