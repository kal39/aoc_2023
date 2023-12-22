fn print_map(map: &Vec<Vec<bool>>) {
    map.iter().for_each(|row| {
        row.iter()
            .for_each(|&c| print!("{}", if c { '.' } else { '#' }));
        println!();
    });
    println!();
}

fn main() {
    let map = std::fs::read_to_string("input/day_21.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().map(|c| c != '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // print_map(&map);

    let size = map.len();
    let mut state = vec![vec![false; size]; size];
    state[0][size / 2] = true;

    for _ in 0..100 {
        let mut new_state = vec![vec![false; size]; size];

        for y in 0..size as i32 {
            for x in 0..size as i32 {
                if !state[y as usize][x as usize] {
                    continue;
                }
                for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || nx >= size as i32 || ny < 0 || ny >= size as i32 {
                        continue;
                    }
                    if !map[ny as usize][nx as usize] {
                        continue;
                    }
                    new_state[ny as usize][nx as usize] = true;
                }
            }
        }

        state = new_state;

        print_map(&state);
    }

    let res = state.iter().flatten().filter(|&&c| c).count();
    println!("result: {}", res);
}
