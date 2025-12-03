fn main() {
    let input = include_str!("../input.txt");
    let banks = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&banks));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&banks));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

fn solve(banks: &[Vec<u8>], to_turn: usize) -> i64 {
    let mut res = 0;

    for bank in banks {
        let mut max = vec![0u8; to_turn];
        let mut k = 0;
        for i in 0..max.len() {
            let to_fill = max.len() - i;
            let bank_rest = bank.len() - k;
            let end = k + bank_rest - to_fill + 1;
            for j in k..end {
                if bank[j] == 9 {
                    max[i] = 9;
                    k = j + 1;
                    break;
                }

                if bank[j] > max[i] {
                    max[i] = bank[j];
                    k = j + 1;
                }
            }
        }
        let imax = max.iter().fold(0, |acc, &d| acc * 10 + d as i64);
        res += imax;
    }

    res
}

fn part1(banks: &[Vec<u8>]) -> i64 {
    solve(banks, 2)
}

fn part2(banks: &[Vec<u8>]) -> i64 {
    solve(banks, 12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../example.txt");
        let banks = parse(input);
        assert_eq!(part1(&banks), 357);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../example.txt");
        let banks = parse(input);
        assert_eq!(part2(&banks), 3121910778619);
    }
}
