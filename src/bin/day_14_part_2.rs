fn tilt(map: &mut Vec<Vec<char>>, dir: (i32, i32)) -> bool {
    let mut updated = false;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                let id = i as i32 + dir.0;
                let jd = j as i32 + dir.1;
                if id >= 0 && jd >= 0 && id < map.len() as i32 && jd < map[0].len() as i32 {
                    if map[id as usize][jd as usize] == '.' {
                        map[id as usize][jd as usize] = 'O';
                        map[i][j] = '.';
                        updated = true;
                    }
                }
            }
        }
    }
    updated
}

fn cycle(map: &mut Vec<Vec<char>>) {
    while tilt(map, (-1, 0)) {}
    while tilt(map, (0, -1)) {}
    while tilt(map, (1, 0)) {}
    while tilt(map, (0, 1)) {}
}

fn main() {
    let mut map = std::fs::read_to_string("input/day_14.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut maps = Vec::new();
    loop {
        if maps.contains(&map) {
            let cycle_start = maps.iter().position(|s| s == &map).unwrap();
            let cycle_len = maps.len() - cycle_start;
            let final_idx = (1000000000 - cycle_start) % cycle_len + cycle_start;

            let mut res = 0;
            for i in 0..maps[final_idx].len() {
                for j in 0..maps[final_idx][i].len() {
                    if maps[final_idx][i][j] == 'O' {
                        res += maps[final_idx].len() - i;
                    }
                }
            }

            println!("result: {}", res);
            break;
        }

        maps.push(map.clone());
        cycle(&mut map);
    }
}
