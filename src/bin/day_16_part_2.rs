use std::collections::HashSet;

fn get(map: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    if i >= 0 && i < map.len() as i32 && j >= 0 && j < map[i as usize].len() as i32 {
        map[i as usize][j as usize]
    } else {
        '.'
    }
}

fn set(map: &mut Vec<Vec<HashSet<char>>>, i: i32, j: i32, c: char) {
    if i >= 0 && i < map.len() as i32 && j >= 0 && j < map[i as usize].len() as i32 {
        map[i as usize][j as usize].insert(c);
    }
}

fn main() {
    let map = std::fs::read_to_string("input/day_16.txt")
        .unwrap()
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max = 0;
    let size = map.len();
    for i in 0..size {
        for j in 0..size {
            if i == 0 || i == size - 1 || j == 0 || j == size - 1 {
                let mut state = map
                    .iter()
                    .map(|v| {
                        v.iter()
                            .map(|_| HashSet::new())
                            .collect::<Vec<HashSet<char>>>()
                    })
                    .collect::<Vec<Vec<HashSet<char>>>>();

                if i == 0 {
                    state[i][j].insert('v');
                }

                if i == map.len() - 1 {
                    state[i][j].insert('^');
                }

                if j == 0 {
                    state[i][j].insert('>');
                }

                if j == map[0].len() - 1 {
                    state[i][j].insert('<');
                }

                let mut changed = true;
                while changed {
                    let mut new_state = state.clone();

                    for i in 0..state.len() {
                        for j in 0..state[i].len() {
                            let i = i as i32;
                            let j = j as i32;

                            state[i as usize][j as usize].iter().for_each(|c| match c {
                                '>' => match get(&map, i, j) {
                                    '.' => set(&mut new_state, i, j + 1, '>'),
                                    '/' => set(&mut new_state, i - 1, j, '^'),
                                    '\\' => set(&mut new_state, i + 1, j, 'v'),
                                    '|' => {
                                        set(&mut new_state, i - 1, j, '^');
                                        set(&mut new_state, i + 1, j, 'v');
                                    }
                                    '-' => set(&mut new_state, i, j + 1, '>'),
                                    _ => panic!(),
                                },
                                '<' => match get(&map, i, j) {
                                    '.' => set(&mut new_state, i, j - 1, '<'),
                                    '/' => set(&mut new_state, i + 1, j, 'v'),
                                    '\\' => set(&mut new_state, i - 1, j, '^'),
                                    '|' => {
                                        set(&mut new_state, i - 1, j, '^');
                                        set(&mut new_state, i + 1, j, 'v');
                                    }
                                    '-' => set(&mut new_state, i, j - 1, '<'),
                                    _ => panic!(),
                                },
                                '^' => match get(&map, i, j) {
                                    '.' => set(&mut new_state, i - 1, j, '^'),
                                    '/' => set(&mut new_state, i, j + 1, '>'),
                                    '\\' => set(&mut new_state, i, j - 1, '<'),
                                    '|' => set(&mut new_state, i - 1, j, '^'),
                                    '-' => {
                                        set(&mut new_state, i, j - 1, '<');
                                        set(&mut new_state, i, j + 1, '>');
                                    }
                                    _ => panic!(),
                                },
                                'v' => match get(&map, i, j) {
                                    '.' => set(&mut new_state, i + 1, j, 'v'),
                                    '/' => set(&mut new_state, i, j - 1, '<'),
                                    '\\' => set(&mut new_state, i, j + 1, '>'),
                                    '|' => set(&mut new_state, i + 1, j, 'v'),
                                    '-' => {
                                        set(&mut new_state, i, j - 1, '<');
                                        set(&mut new_state, i, j + 1, '>');
                                    }
                                    _ => panic!(),
                                },
                                '.' => (),
                                c => panic!("found {} at {} {}", c, i, j),
                            });
                        }
                    }

                    changed = false;
                    for i in 0..state.len() {
                        for j in 0..state[i].len() {
                            if state[i][j] != new_state[i][j] {
                                changed = true;
                            }
                        }
                    }

                    state = new_state;
                }

                let mut res = 0;
                for i in 0..state.len() {
                    for j in 0..state[i].len() {
                        if state[i][j].len() > 0 {
                            res += 1;
                        }
                    }
                }

                println!("{} {} -> {}", i, j, res);
                if res > max {
                    max = res;
                }
            }
        }
    }

    println!("result: {}", max);
}
