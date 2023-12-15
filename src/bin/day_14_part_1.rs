fn main() {
    let mut map = std::fs::read_to_string("input/day_14.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut updated = true;
    while updated {
        updated = false;
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 'O' {
                    if i > 0 && map[i - 1][j] == '.' {
                        map[i - 1][j] = 'O';
                        map[i][j] = '.';
                        updated = true;
                    }
                }
            }
        }
    }

    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                res += map.len() - i;
            }
        }
    }

    println!("result: {}", res);
}
