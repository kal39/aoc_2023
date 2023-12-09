fn deriv(seq: &Vec<i32>) -> i32 {
    if seq.iter().all(|v| *v == 0) {
        return 0;
    } else {
        let mut d = Vec::new();
        for i in 0..seq.len() - 1 {
            d.push(seq[i + 1] - seq[i]);
        }
        return d.first().unwrap() - deriv(&d);
    }
}

fn main() {
    let res = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let seq = l
                .split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            seq.first().unwrap() - deriv(&seq)
        })
        .sum::<i32>();

    println!("result: {}", res);
}
