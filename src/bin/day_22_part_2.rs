use itertools::Itertools;
use std::vec;

fn count_affected(supported: Vec<Vec<usize>>) -> usize {
    // println!("{:?}", supported);

    let remaining = supported
        .iter()
        .map(|i| {
            i.iter()
                .map(|&j| j)
                .filter(|&j| j == 0 || supported[j - 1].len() != 0)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if remaining == supported {
        remaining.iter().filter(|&i| i.len() == 0).count() - 1
    } else {
        count_affected(remaining)
    }
}

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

    let mut supported = vec![vec![]; bricks.len()];

    for id in 1..bricks.len() + 1 {
        for x in 0..max_val.0 + 1 {
            for y in 0..max_val.1 + 1 {
                for z in 0..max_val.2 + 1 {
                    if state[x][y][z] == id {
                        if z != 1 && state[x][y][z - 1] != 0 && state[x][y][z - 1] != id {
                            supported[id - 1].push(state[x][y][z - 1]);
                        }
                    }
                }
            }
        }
    }

    supported = supported
        .iter()
        .map(|i| if i.len() == 0 { vec![0] } else { i.clone() })
        .collect::<Vec<_>>();

    let res = (1..bricks.len() + 1)
        .map(|id| {
            let mut init = supported.clone();
            init[id - 1] = vec![];
            count_affected(init)
        })
        .sum::<usize>();

    println!("result: {}", res);
}
