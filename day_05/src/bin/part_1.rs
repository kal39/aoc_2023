fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Error reading file");

    let mut seeds = input
        .lines()
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| {
            let id = s.parse::<u32>().unwrap();
            (id, id)
        })
        .collect::<Vec<(u32, u32)>>();

    input.split("\n\n").skip(1).for_each(|m| {
        println!("\n{}\n", m.lines().nth(0).unwrap());

        let map = m
            .lines()
            .skip(1)
            .filter(|l| !l.is_empty())
            .map(|l| {
                let vals = l
                    .split_whitespace()
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                (vals[0], vals[1], vals[2])
            })
            .collect::<Vec<(u32, u32, u32)>>();

        for s in &mut seeds {
            for &(dst, src, len) in &map {
                if src <= s.1 && s.1 < src + len {
                    let overlap = s.1 - src;
                    println!("{} -> {}", s.0, dst + overlap);
                    *s = (s.0, dst + overlap);
                    break;
                }
            }
        }

        println!("\n{:?}\n", seeds);
    });

    let res = seeds.iter().min_by(|x, y| x.1.cmp(&y.1)).unwrap().1;

    println!("result: {}", res);
}
