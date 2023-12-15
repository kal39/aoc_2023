use std::collections::HashMap;

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
    let mut boxes: HashMap<u64, Vec<(String, u32)>> = HashMap::new();

    std::fs::read_to_string("input/day_15.txt")
        .unwrap()
        .trim()
        .split(",")
        .for_each(|s| {
            let label = if s.contains('=') {
                s[0..s.len() - 2].to_string()
            } else {
                s[0..s.len() - 1].to_string()
            };

            let box_id = hash(&label);
            let box_ref = match boxes.get_mut(&box_id) {
                Some(b) => b,
                None => {
                    boxes.insert(box_id, Vec::new());
                    boxes.get_mut(&box_id).unwrap()
                }
            };

            if s.contains('=') {
                let focal_length = s.chars().last().unwrap().to_digit(10).unwrap();
                if box_ref.iter().any(|(l, _)| *l == label) {
                    box_ref.iter_mut().for_each(|(l, f)| {
                        if *l == label {
                            *f = focal_length;
                        }
                    });
                } else {
                    box_ref.push((label, focal_length as u32));
                }
            } else {
                box_ref.retain(|(l, _)| *l != label);
            }
        });

    let res = boxes
        .iter()
        .map(|(&k, v)| {
            v.iter()
                .enumerate()
                .map(|(i, (_, f))| (k + 1) * (i as u64 + 1) * *f as u64)
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("result: {}", res);
}
