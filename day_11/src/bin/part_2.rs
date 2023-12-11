fn main() {
    let mut coords = Vec::new();

    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, l)| {
            l.chars().enumerate().for_each(|(j, c)| {
                if c == '#' {
                    coords.push((i as i64, j as i64));
                }
            });
        });

    println!("{:?}", coords);

    let rows = coords.iter().map(|c| c.0).max().unwrap();
    let cols = coords.iter().map(|c| c.1).max().unwrap();

    for i in (0..rows).rev() {
        if coords.iter().all(|&c| c.0 != i) {
            coords.iter_mut().for_each(|c| {
                if c.0 > i {
                    c.0 += 1000000 - 1;
                }
            });
        }
    }

    for i in (0..cols).rev() {
        if coords.iter().all(|&c| c.1 != i) {
            coords.iter_mut().for_each(|c| {
                if c.1 > i {
                    c.1 += 1000000 - 1;
                }
            });
        }
    }

    println!("{:?}", coords);

    let mut res = 0;
    for i in 0..coords.len() {
        for j in (i + 1)..coords.len() {
            res += (coords[i].0 - coords[j].0).abs() + (coords[i].1 - coords[j].1).abs();
        }
    }

    println!("result: {}", res);
}
