fn hash(s: &str) -> u64 {
    let mut res = 0;
    s.chars().for_each(|c| res = ((res + c as u64) * 17) % 256);
    res
}

fn main() {
    let mut boxes: Vec<Vec<(String, u32)>> = (0..256).map(|_| Vec::new()).collect();

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
            let box_ref = boxes.get_mut(hash(&label) as usize).unwrap();

            if s.contains('=') {
                let focal_length = s.chars().last().unwrap().to_digit(10).unwrap();
                if box_ref.iter().any(|(l, _)| *l == label) {
                    box_ref
                        .iter_mut()
                        .filter(|(l, _)| *l == label)
                        .for_each(|(_, f)| *f = focal_length);
                } else {
                    box_ref.push((label, focal_length as u32));
                }
            } else {
                box_ref.retain(|(l, _)| *l != label);
            }
        });

    let res = boxes
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(|(j, (_, f))| (i as u64 + 1) * (j as u64 + 1) * (*f as u64))
                .sum::<u64>()
        })
        .sum::<u64>();

    println!("result: {}", res);
}
