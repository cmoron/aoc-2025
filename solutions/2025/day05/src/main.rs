fn main() {
    let input = include_str!("../input.txt");
    let (ranges, ids) = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&ranges, &ids));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&ranges));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges_str, ids_str) = input
        .split_once("\n\n").unwrap();

    let ranges = ranges_str
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("-").unwrap();
            let start = start.parse::<usize>().unwrap();
            let end = end.parse::<usize>().unwrap();
            (start, end)
        })
        .collect();
    let ids: Vec<usize> = ids_str
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect();

    (ranges, ids)
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> usize {
    let mut res = 0;

    for &id in ids {
        for &(start, end) in ranges {
            if id >= start && id <= end {
                res += 1;
                break;
            }
        }
    }

    res
}

fn part2(ranges: &[(usize, usize)]) -> usize {
    let mut res = 0;

    let mut ranges = ranges.to_vec();

    ranges.sort_by_key(|&(start, _)| start);

    let mut prev_max = 0;

    for index in 0..ranges.len() - 1 {
        let (_, end) = ranges[index];
        prev_max = prev_max.max(end);

        let next = &mut ranges[index + 1];

        if prev_max >= next.0 {
            if prev_max + 1 > next.1 {
                next.0 = 0;
                next.1 = 0;
            } else {
                next.0 = prev_max + 1;
            }
        }
    }

    for (start, end) in ranges {
        if end != 0 {
            res += end - start + 1;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        let (ranges, ids) = parse(example_input);
        assert_eq!(part1(&ranges, &ids), 3);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let (ranges, _) = parse(example_input);
        assert_eq!(part2(&ranges), 14);
    }
}
