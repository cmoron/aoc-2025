use std::collections::HashMap;

type Graph = HashMap<&'static str, Vec<&'static str>>;

fn main() {
    let input: &'static str = include_str!("../input.txt");
    let devices = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&devices));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let input: &'static str = include_str!("../input.txt");
    let devices = parse(input);
    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&devices));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &'static str) -> Graph {
    let mut devices = HashMap::new();

    for line in input.lines() {
        let (device, connections_str) = line.split_once(": ").unwrap();
        let connections: Vec<&'static str> = connections_str.split_whitespace().collect();
        devices.insert(device, connections);
    }

    devices
}

fn dp(start: &'static str, devices: &Graph, memo: &mut HashMap<&'static str, usize>) -> usize {
    if start == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(start) {
        return cached;
    }

    let total = devices[start].iter().map(|n| dp(n, devices, memo)).sum();

    memo.insert(start, total);

    total
}

fn dp2(
    start: &'static str,
    devices: &Graph,
    memo: &mut HashMap<&'static str, (usize, usize, usize, usize)>,
) -> (usize, usize, usize, usize) {

    if let Some(&res) = memo.get(start) {
        return res;
    }

    if start == "out" {
        let res = (1, 0, 0, 0);
        memo.insert(start, res);
        return res;
    }

    let mut none = 0usize;
    let mut dac_only = 0usize;
    let mut fft_only = 0usize;
    let mut both = 0usize;

    for &node in &devices[start] {
        let (n, d, f, b) = dp2(node, devices, memo);
        none += n;
        dac_only += d;
        fft_only += f;
        both += b;
    }

    if start == "dac" {
        let new_dac_only = none + dac_only;
        let new_both = fft_only + both;
        none = 0;
        dac_only = new_dac_only;
        fft_only = 0;
        both = new_both;
    }

    if start == "fft" {
        let new_fft_only = none + fft_only;
        let new_both = dac_only + both;
        none = 0;
        dac_only = 0;
        fft_only = new_fft_only;
        both = new_both;
    }

    let res = (none, dac_only, fft_only, both);
    memo.insert(start, res);
    res
}

fn part1(devices: &Graph) -> usize {
    let mut memo = HashMap::new();
    dp("you", devices, &mut memo)
}

fn part2(devices: &Graph) -> usize {
    let mut memo = HashMap::new();
    let (_none, _dac_only, _fft_only, both) = dp2("svr", devices, &mut memo);
    both
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input: &'static str = include_str!("../example.txt");
        let devices = parse(example_input);
        assert_eq!(part1(&devices), 5);
    }

    #[test]
    fn test_part2_example() {
        let example_input: &'static str = include_str!("../example2.txt");
        let devices = parse(example_input);
        assert_eq!(part2(&devices), 2);
    }
}
