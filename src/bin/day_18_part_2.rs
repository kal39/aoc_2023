use regex::Regex;
use std::vec;

fn main() {
    let re = Regex::new(r".*\(#(.{5})(.)\)").unwrap();
    let mut vertices = vec![(0, 0)];
    let mut total_len = 0;

    std::fs::read_to_string("input/day_18.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let captures = re.captures(line).unwrap();
            let val = i64::from_str_radix(captures.get(1).unwrap().as_str(), 16).unwrap();
            let dir = captures.get(2).unwrap().as_str().chars().next().unwrap();
            let delta = match dir {
                '0' => (val, 0),
                '2' => (-val, 0),
                '1' => (0, val),
                '3' => (0, -val),
                _ => panic!(),
            };
            vertices.push((
                vertices.last().unwrap().0 + delta.0,
                vertices.last().unwrap().1 + delta.1,
            ));
            total_len += val;
        });

    let result = (0..vertices.len() - 1)
        .map(|i| {
            let (x1, y1) = vertices[i];
            let (x2, y2) = vertices[i + 1];
            x1 * y2 - x2 * y1
        })
        .sum::<i64>()
        .abs()
        / 2
        + total_len / 2
        + 1;

    println!("result: {}", result);
}
