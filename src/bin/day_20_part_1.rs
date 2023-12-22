use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Signal {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    kind: ModuleType,
    outputs: Vec<String>,
    inputs: Vec<String>,
    state: Vec<Signal>,
}

fn send(
    modules: &mut HashMap<String, Module>,
    signals: &mut Vec<(String, String, Signal)>,
    from: String,
    to: String,
    signal: Signal,
) {
    // println!("{} -- {:?} --> {}", from, signal, to);

    let module = modules.get_mut(&to).unwrap();

    match module.kind {
        ModuleType::FlipFlop => {
            if signal == Signal::Low {
                module.state[0] = match module.state[0] {
                    Signal::High => Signal::Low,
                    Signal::Low => Signal::High,
                };
                module.outputs.iter().for_each(|output| {
                    signals.push((to.clone(), output.clone(), module.state[0]));
                });
            }
        }
        ModuleType::Conjunction => {
            let idx = module.inputs.iter().position(|m| *m == from).unwrap();
            module.state[idx] = signal;

            if module.state.iter().all(|s| *s == Signal::High) {
                module.outputs.iter().for_each(|output| {
                    signals.push((to.clone(), output.clone(), Signal::Low));
                });
            } else {
                module.outputs.iter().for_each(|output| {
                    signals.push((to.clone(), output.clone(), Signal::High));
                });
            }
        }
        ModuleType::Broadcast => {
            module.outputs.iter().for_each(|output| {
                signals.push((to.clone(), output.clone(), signal));
            });
        }
        ModuleType::None => {}
    };
}

fn press_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut signals = Vec::new();
    signals.push(("button".to_string(), "broadcaster".to_string(), Signal::Low));

    let mut res = (0, 0);
    while signals.len() > 0 {
        res.0 += signals.iter().filter(|s| s.2 == Signal::Low).count();
        res.1 += signals.iter().filter(|s| s.2 == Signal::High).count();

        let mut new_signals = Vec::new();
        signals.iter().for_each(|(from, to, signal)| {
            send(modules, &mut new_signals, from.clone(), to.clone(), *signal);
        });

        signals = new_signals;
    }

    res
}

fn main() {
    let mut modules = HashMap::new();

    std::fs::read_to_string("input/day_20.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let head = line.split(" -> ").nth(0).unwrap();

            let (name, kind) = match head.chars().nth(0).unwrap() {
                '%' => (head[1..].to_string(), ModuleType::FlipFlop),
                '&' => (head[1..].to_string(), ModuleType::Conjunction),
                _ => (
                    head.to_string(),
                    if head == "broadcaster" {
                        ModuleType::Broadcast
                    } else {
                        ModuleType::None
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

            modules.insert(
                name,
                Module {
                    kind,
                    outputs,
                    inputs: Vec::new(),
                    state: Vec::new(),
                },
            );
        });

    modules.clone().iter().for_each(|(name, module)| {
        module.outputs.iter().for_each(|output| {
            if let Some(module) = modules.get_mut(output) {
                module.inputs.push(name.to_string());
            } else {
                modules.insert(
                    output.to_string(),
                    Module {
                        kind: ModuleType::None,
                        outputs: Vec::new(),
                        inputs: Vec::new(),
                        state: Vec::new(),
                    },
                );
            }
        });
    });

    modules.iter_mut().for_each(|(_, module)| {
        module
            .inputs
            .iter()
            .for_each(|_| module.state.push(Signal::Low));
    });

    let res = (0..1000)
        .map(|_| press_button(&mut modules))
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .unwrap();

    let res = res.0 * res.1;

    println!("result: {:?}", res);
}
