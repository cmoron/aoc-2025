use std::collections::VecDeque;

use good_lp::{
    variables, variable, SolverModel, Solution, Expression,
    solvers::microlp::microlp,
};

fn main() {
    let input = include_str!("../input.txt");
    let machines = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&machines));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&machines));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

#[derive(Debug)]
struct Machine {
    target_mask: u64,
    buttons: Vec<u64>,
    joltage: Vec<u16>,
    nb_lights: usize,
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
            let joltage: Vec<u16> = parsed_joltage_str
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();

            Machine {
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

fn solve2(m: &Machine) -> Option<usize> {

    let num_buttons = m.buttons.len();
    let num_counters = m.joltage.len();

    // Variables x_j = nombre de pressions sur chaque bouton
    let mut vars = variables!();
    let press_vars: Vec<_> = (0..num_buttons)
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    // Objectif : minimiser la somme des pressions
    let objective: Expression = press_vars.iter().copied().sum();
    let mut problem = vars.minimise(objective).using(microlp);

    // Contraintes A x = joltage
    for i in 0..num_counters {
        let mut expr: Expression = 0.0.into();
        for (btn_idx, &mask) in m.buttons.iter().enumerate() {
            if (mask >> i) & 1 == 1 {
                expr = expr + press_vars[btn_idx];
            }
        }
        problem = problem.with(expr.eq(m.joltage[i] as f64));
    }

    let sol = problem.solve().ok()?;

    Some(
        press_vars
            .iter()
            .map(|&v| sol.value(v).round() as usize)
            .sum(),
    )
}

fn part1(machines: &[Machine]) -> usize {
    machines.iter().map(|m| solve(m).unwrap()).sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines.iter().map(|m| solve2(m).unwrap()).sum()
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

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let machines = parse(example_input);
        assert_eq!(part2(&machines), 33);
    }
}
