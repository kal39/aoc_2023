fn main() {
    let input = std::fs::read_to_string("input/day_05.txt").expect("Error reading file");

    let maps = input
        .split("\n\n")
        .skip(1)
        .map(|m| {
            m.lines()
                .skip(1)
                .filter(|l| !l.is_empty())
                .map(|l| {
                    let vals = l
                        .split_whitespace()
                        .map(|n| n.parse::<u64>().unwrap())
                        .collect::<Vec<u64>>();
                    (vals[0], vals[1], vals[2])
                })
                .collect::<Vec<(u64, u64, u64)>>()
        })
        .collect::<Vec<Vec<(u64, u64, u64)>>>();

    let mut min_loc = u64::MAX;

    input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(2)
        .for_each(|c| {
            let start = c[0].parse::<u64>().unwrap();
            let len = c[1].parse::<u64>().unwrap();

            for seed in start..start + len {
                let mut cur = seed;

                for m in &maps {
                    for &(dst, src, len) in m {
                        if src <= cur && cur < src + len {
                            let overlap = cur - src;
                            cur = dst + overlap;
                            break;
                        }
                    }
                }

                if cur < min_loc {
                    min_loc = cur;
                }
            }
        });

    println!("result: {}", min_loc);
}
