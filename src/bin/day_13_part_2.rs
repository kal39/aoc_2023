fn main() {
    let res = std::fs::read_to_string("input/day_13.txt")
        .unwrap()
        .split("\n\n")
        .map(|m| {
            let m = m
                .lines()
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            let mut mirror_row = 0;
            let mut mirror_col = 0;

            for i in 1..m.len() {
                let mut mismatches = 0;
                for j in 0..usize::min(i, m.len() - i) {
                    for k in 0..m[0].len() {
                        if m[i - j - 1][k] != m[i + j][k] {
                            mismatches += 1;
                        }
                    }
                }

                if mismatches == 1 {
                    mirror_row = i;
                }
            }

            for i in 1..m[0].len() {
                let mut mismatches = 0;
                for j in 0..usize::min(i, m[0].len() - i) {
                    for k in 0..m.len() {
                        if m[k][i - j - 1] != m[k][i + j] {
                            mismatches += 1;
                        }
                    }
                }

                if mismatches == 1 {
                    mirror_col = i;
                }
            }

            (mirror_row, mirror_col)
        })
        .fold(0, |acc, (r, c)| acc + 100 * r + c);

    println!("result: {}", res);
}
