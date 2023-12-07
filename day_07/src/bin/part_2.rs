const CARDS: &str = "J23456789TQKA";

fn card_strength(c: char) -> u32 {
    CARDS.find(c).unwrap() as u32
}

fn hand_strength(s: &String) -> u32 {
    let jokers = s.matches("J").count();
    let s = s.replace("J", "");

    let mut occurrences = CARDS
        .chars()
        .into_iter()
        .map(|c| s.matches(c).count())
        .collect::<Vec<usize>>();

    occurrences.sort();
    occurrences.reverse();
    occurrences[0] += jokers;

    match occurrences.as_slice() {
        [5, ..] => 7,
        [4, 1, ..] => 6,
        [3, 2, ..] => 5,
        [3, 1, 1, ..] => 4,
        [2, 2, 1, ..] => 3,
        [2, 1, 1, 1, ..] => 2,
        [1, 1, 1, 1, 1, ..] => 1,
        _ => panic!(),
    }
}

fn main() {
    let mut hands = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            let hand = l.split(" ").nth(0).unwrap().to_string();
            let bet = l.split(" ").nth(1).unwrap().parse::<u32>().unwrap();
            (hand, bet)
        })
        .collect::<Vec<(String, u32)>>();

    hands.sort_by(|hand_a, hand_b| {
        let strength = hand_strength(&hand_a.0).cmp(&hand_strength(&hand_b.0));
        if strength == std::cmp::Ordering::Equal {
            for i in 0..5 {
                let card_a = hand_a.0.chars().nth(i).unwrap();
                let card_b = hand_b.0.chars().nth(i).unwrap();
                if card_a != card_b {
                    return card_strength(card_a).cmp(&card_strength(card_b));
                }
            }
        }
        return strength;
    });

    let res = hands
        .iter()
        .enumerate()
        .map(|(rank, h)| (rank + 1) as u32 * h.1)
        .sum::<u32>();

    println!("result: {}", res);
}
