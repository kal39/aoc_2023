fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    let mut copies = vec![1; input.len()];

    input.iter().enumerate().for_each(|(i, line)| {
        let winning = line
            .split("|")
            .nth(0)
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        let matches = line
            .split("|")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| {
                let val = s.parse::<u32>().unwrap();
                winning.contains(&val) as u32
            })
            .sum::<u32>();

        for j in (i + 1)..(i + matches as usize + 1) {
            copies[j] = copies[j] + 1 * copies[i];
        }
    });

    let res = copies.iter().sum::<u32>();

    println!("result: {}", res);
}
