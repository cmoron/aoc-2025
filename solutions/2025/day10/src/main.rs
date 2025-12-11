use std::collections::VecDeque;

fn main() {
    // let input = include_str!("../example.txt");
    let input = include_str!("../input.txt");
    let machines = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&machines));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

#[derive(Debug)]
struct Machine {
    target: Vec<char>,
    target_mask: u64,
    buttons: Vec<u64>,
    joltage: Vec<usize>,
    nb_lights: usize,
}

fn print_machines(machine: &[Machine]) {
    for (i, m) in machine.iter().enumerate() {
        println!("Machine {}:", i + 1);
        println!("  Target: {}", m.target.iter().collect::<String>());
        println!(
            "  Target_mask: {:0width$b}",
            m.target_mask,
            width = m.nb_lights
        );
        print!("  Buttons: [");
        for btn in &m.buttons[..m.buttons.len() - 1] {
            print!("{:0b}, ", btn);
        }
        print!("{:0b}]", m.buttons.last().unwrap());
        println!("  Joltage: {:?}", m.joltage);
        println!("  Nb lights: {:?}", m.nb_lights);
        println!();
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            let (target_str, rest_str) = line.split_once(' ').unwrap();
            let target_str = &target_str[1..target_str.len() - 1];

            // Convert target to bitmask representation
            let target_mask = target_str
                .chars()
                .enumerate()
                .fold(0u64, |mask, (i, c)| mask | ((c == '#') as u64) << i);
            let (buttons_str, joltage_str) = rest_str.split_once('{').unwrap();
            let parsed_joltage_str = joltage_str.replace('}', "");
            let target: Vec<char> = target_str.chars().collect();
            let nb_lights = target.len();
            let buttons: Vec<Vec<usize>> = buttons_str
                .trim()
                .split_whitespace()
                .map(|button_str| {
                    let parsed_buttons_str = button_str.replace("(", "").replace(")", "");
                    let button: Vec<usize> = parsed_buttons_str
                        .split(',')
                        .map(|b| b.parse().unwrap())
                        .collect();
                    button
                })
                .collect();

            // Convert buttons to bitmask representation
            let buttons_mask = buttons
                .iter()
                .map(|btn| btn.iter().fold(0u64, |mask, &idx| mask | (1u64 << idx)))
                .collect();
            let joltage: Vec<usize> = parsed_joltage_str
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();

            Machine {
                target: target,
                target_mask: target_mask,
                buttons: buttons_mask,
                joltage: joltage,
                nb_lights: nb_lights,
            }
        })
        .collect()
}

fn solve(m: &Machine) -> Option<usize> {
    // BFS to find the minimum number of button presses to reach the target state
    let target = m.target_mask;
    let start: u64 = 0;
    let max_state = 1usize << m.nb_lights;
    let mut iterations = vec![u16::MAX; max_state];

    if target == start {
        return Some(0);
    }

    let mut q = VecDeque::new();

    iterations[start as usize] = 0;
    q.push_back(start);

    while let Some(state) = q.pop_front() {
        let it = iterations[state as usize];
        for &btn in &m.buttons {
            let next = state ^ btn;
            let idx = next as usize;
            if iterations[idx] != u16::MAX {
                continue;
            }

            iterations[idx] = it + 1;

            if next == target {
                return Some(iterations[idx] as usize);
            }

            q.push_back(next);
        }
    }

    None
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m| solve(m).unwrap()).sum()
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        let machines = parse(example_input);
        assert_eq!(part1(&machines), 7);
    }
}
