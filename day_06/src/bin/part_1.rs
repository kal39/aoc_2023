fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let times = input[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    let distances = input[1]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<f32>().unwrap())
        .collect::<Vec<f32>>();

    let res = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            let lower_bound = (t - f32::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0;
            let upper_bound = (t + f32::sqrt(t.powf(2.0) - 4.0 * d)) / 2.0;

            let lower_bound = lower_bound.floor() as i32 + 1;
            let upper_bound = upper_bound.ceil() as i32 - 1;

            upper_bound - lower_bound + 1
        })
        .reduce(|acc, x| acc * x)
        .unwrap();

    print!("result: {}", res);
}
