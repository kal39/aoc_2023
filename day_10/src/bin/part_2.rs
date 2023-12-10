use std::{collections::HashMap, vec};

fn main() {
    let mut chars: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    chars.insert('|', vec![(-1, 0), (1, 0)]);
    chars.insert('-', vec![(0, -1), (0, 1)]);
    chars.insert('L', vec![(-1, 0), (0, 1)]);
    chars.insert('J', vec![(-1, 0), (0, -1)]);
    chars.insert('7', vec![(1, 0), (0, -1)]);
    chars.insert('F', vec![(1, 0), (0, 1)]);
    chars.insert('.', vec![]);
    chars.insert('S', vec![(-1, 0), (1, 0), (0, -1), (0, 1)]);

    let mut map = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut state = vec![vec![-1; map[0].len()]; map.len()];

    let mut start_pos = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start_pos = (i, j);
            }
        }
    }

    state[start_pos.0][start_pos.1] = 0;

    let mut iter = 0;
    let mut state_changed = true;
    let mut start_dirs = Vec::new();

    while state_changed {
        state_changed = false;

        for i in 0..(map.len() as i32) {
            for j in 0..(map[i as usize].len() as i32) {
                if state[i as usize][j as usize] == iter {
                    chars
                        .get(&map[i as usize][j as usize])
                        .unwrap()
                        .iter()
                        .for_each(|(di, dj)| {
                            let (i, j) = (i + di, j + dj);

                            if i < 0
                                || j < 0
                                || i >= state.len() as i32
                                || j >= state[0].len() as i32
                            {
                                return;
                            }

                            if state[i as usize][j as usize] != -1 {
                                return;
                            }

                            if !chars
                                .get(&map[i as usize][j as usize])
                                .unwrap()
                                .contains(&(-di, -dj))
                            {
                                return;
                            }

                            state[i as usize][j as usize] = iter + 1;
                            state_changed = true;

                            //  save start dirs
                            if iter == 0 {
                                start_dirs.push((*di, *dj));
                            }
                        });
                }
            }
        }

        iter += 1;
    }

    for c in "|-LJF7".chars() {
        let dirs = chars.get(&c).unwrap();
        if dirs == start_dirs.as_slice() {
            map[start_pos.0][start_pos.1] = c;
        }
    }

    let mut count = 0;
    for i in 0..state.len() {
        let l = state[i]
            .iter()
            .zip(map[i].iter())
            .map(|(s, m)| match *s {
                -1 => '.',
                _ => *m,
            })
            .collect::<String>();

        let l = l
            .replace("-", "")
            .replace("L7", "|")
            .replace("FJ", "|")
            .replace("F7", "")
            .replace("LJ", "");

        let mut inside = false;
        for c in l.chars() {
            if c == '|' {
                inside = !inside;
            } else if inside && c == '.' {
                count += 1;
            }
        }
    }

    print!("result: {}", count);
}
