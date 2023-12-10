use std::collections::HashMap;

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

    let map = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut state = vec![vec![-1; map[0].len()]; map.len()];

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                state[i][j] = 0;
            }
        }
    }

    let mut iter = 0;
    let mut state_changed = true;

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
                        });
                }
            }
        }

        iter += 1;
    }

    let res = state.iter().flatten().max().unwrap();

    println!("result: {}", res);
}
