fn main() {
    let input = include_str!("../input.txt");
    let diagram = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&diagram));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&diagram));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(diagram: &[Vec<char>]) -> usize {
    let mut res = 0;
    let width = diagram[0].len();
    let start_index = diagram[0].iter().position(|&s| s == 'S').unwrap();
    let mut beams = vec![false; width];
    beams[start_index] = true;

    for line in diagram[1..].iter() {
        if line.iter().all(|&c| c == '.') {
            continue;
        }

        for (i, &c) in line.iter().enumerate() {
            if c == '^' && beams[i] {
                if i > 0 {
                    beams[i - 1] = true;
                }

                if i + 1 < width {
                    beams[i + 1] = true;
                }

                res += 1;
                beams[i] = false;
            }
        }
    }

    res
}

fn dp(
    diagram: &[Vec<char>],
    line_index: usize,
    beam_index: usize,
    cache: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    let rows = diagram.len();
    let cols = diagram[0].len();

    if let Some(cached_res) = cache[line_index][beam_index] {
        return cached_res;
    }

    let line = &diagram[line_index];
    let mut res = 0;

    // Pas de splitter sur la dernière ligne.
    if line_index == rows - 2 {
        if line[beam_index] == '^' {
            res = 1;
        }
    } else {
        if line[beam_index] == '^' {
            res += 1;

            if beam_index > 0 {
                res += dp(diagram, line_index + 1, beam_index - 1, cache);
            }

            if beam_index + 1 < cols {
                res += dp(diagram, line_index + 1, beam_index + 1, cache);
            }
        } else {
            res += dp(diagram, line_index + 1, beam_index, cache);
        }
    }

    cache[line_index][beam_index] = Some(res);

    res
}

fn part2(diagram: &[Vec<char>]) -> usize {
    let width = diagram[0].len();
    let height = diagram.len();
    let start_index = diagram[0].iter().position(|&s| s == 'S').unwrap();

    let mut cache = vec![vec![None; width]; height];

    // 1 timeline de départ en partant de S
    // La timeline se duplique à chaque splitter '^'
    1 + dp(diagram, 1, start_index, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        let diagram = parse(example_input);
        assert_eq!(part1(&diagram), 21);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let diagram = parse(example_input);
        assert_eq!(part2(&diagram), 40);
    }
}
