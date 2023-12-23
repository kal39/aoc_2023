use itertools::Itertools;
use std::vec;

fn main() {
    let bricks = std::fs::read_to_string("input/day_22.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('~');
            let p1 = parts
                .next()
                .unwrap()
                .split(',')
                .map(|p| p.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap();
            let p2 = parts
                .next()
                .unwrap()
                .split(',')
                .map(|p| p.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap();

            (p1, p2)
        })
        .collect::<Vec<_>>();

    let max_val = bricks.iter().fold((0, 0, 0), |acc, (p1, p2)| {
        (
            usize::max(acc.0, usize::max(p1.0, p2.0)),
            usize::max(acc.1, usize::max(p1.1, p2.1)),
            usize::max(acc.2, usize::max(p1.2, p2.2)),
        )
    });

    let mut state = vec![vec![vec![0; max_val.2 + 1]; max_val.1 + 1]; max_val.0 + 1];
    bricks.iter().enumerate().for_each(|(i, (p1, p2))| {
        for x in p1.0..p2.0 + 1 {
            for y in p1.1..p2.1 + 1 {
                for z in p1.2..p2.2 + 1 {
                    state[x][y][z] = i + 1;
                }
            }
        }
    });

    let mut changed = true;
    while changed {
        changed = false;

        for id in 1..bricks.len() + 1 {
            let mut can_fall = true;

            for x in 0..max_val.0 + 1 {
                for y in 0..max_val.1 + 1 {
                    for z in 0..max_val.2 + 1 {
                        if state[x][y][z] == id {
                            if z == 1 || (state[x][y][z - 1] != 0 && state[x][y][z - 1] != id) {
                                can_fall = false;
                            }
                        }
                    }
                }
            }

            if !can_fall {
                continue;
            }

            for x in 0..max_val.0 + 1 {
                for y in 0..max_val.1 + 1 {
                    for z in 0..max_val.2 + 1 {
                        if state[x][y][z] == id {
                            state[x][y][z] = 0;
                            state[x][y][z - 1] = id;
                        }
                    }
                }
            }

            changed = true;
        }
    }

    let mut supporting = vec![vec![]; bricks.len()];
    for id in 1..bricks.len() + 1 {
        for x in 0..max_val.0 + 1 {
            for y in 0..max_val.1 + 1 {
                for z in 0..max_val.2 + 1 {
                    if state[x][y][z] == id {
                        if z != 1 && state[x][y][z - 1] != 0 && state[x][y][z - 1] != id {
                            supporting[state[x][y][z - 1] - 1].push(id);
                        }
                    }
                }
            }
        }
    }

    let supporting_flat = supporting
        .iter()
        .map(|s| s.iter().unique().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();

    let res = supporting
        .iter()
        .map(|s| {
            s.iter()
                .map(|id| supporting_flat.iter().filter(|&&i| i == id).count())
                .any(|c| c == 1)
        })
        .filter(|&b| !b)
        .count();

    println!("result: {}", res);
}
