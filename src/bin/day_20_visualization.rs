fn main() {
    let mut out = String::new();
    out.push_str("digraph {\n");

    std::fs::read_to_string("input/day_20.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let head = line.split(" -> ").nth(0).unwrap();

            let (name, kind) = match head.chars().nth(0).unwrap() {
                '%' => (head[1..].to_string(), "flip-flop"),
                '&' => (head[1..].to_string(), "conjunction"),
                _ => (
                    head.to_string(),
                    if head == "broadcaster" {
                        "broadcast"
                    } else {
                        "none"
                    },
                ),
            };

            let outputs = line
                .split(" -> ")
                .nth(1)
                .unwrap()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            out.push_str(&format!(
                "    \"{}\" [shape=\"{}\"];\n",
                name,
                match kind {
                    "flip-flop" => "box",
                    "conjunction" => "diamond",
                    "broadcast" => "octagon",
                    _ => "doublecircle",
                }
            ));

            outputs.iter().for_each(|output| {
                out.push_str(&format!("    \"{}\" -> \"{}\";\n", name, output));
            });
        });

    out.push_str("}\n");

    std::fs::write("output/day_20.dot", out).unwrap();
}
