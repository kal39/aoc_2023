use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;

    let out = input
        .split("\n")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>())
        .map(|s1| {
            10 * s1.first().unwrap().to_digit(10).unwrap()
                + s1.last().unwrap().to_digit(10).unwrap()
        })
        .sum::<u32>()
        .to_string();

    println!("result: {}", out);
    Ok(())
}
