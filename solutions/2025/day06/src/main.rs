fn main() {
    let input = include_str!("../input.txt");
    let problems = parse_p1(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&problems));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    let operations = parse_p2(input);
    println!("Part 2: {}", part2(&operations));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse_p1(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn parse_p2(input: &str) -> Vec<Vec<char>> {
    let problems = input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let r = problems.len();
    let c = problems[0].len();
    let mut operation_lines = vec![vec![' '; r]; c];

    for i in 0..r {
        for j in 0..c {
            operation_lines[j][i] = problems[i][j];
        }
    }

    operation_lines
}

fn part1(input: &[Vec<&str>]) -> usize {
    let mut res = 0;
    let r = input.len();
    let c = input[0].len();
    let mut ops = vec![vec![""; r]; c];

    for i in 0..r {
        for j in 0..c {
            ops[j][i] = input[i][j];
        }
    }

    for op in &ops {
        let operator = op[op.len() - 1];
        let operands = &op[..op.len() - 1]
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let val = match operator {
            "+" => operands.iter().fold(0, |acc, &x| acc + x),
            "*" => operands.iter().fold(1, |acc, &x| acc * x),
            &_ => unreachable!("Unexpected operator in part1"),
        };

        res += val;
    }

    res
}

fn calc(operator: char, values: &[u32]) -> u64 {
    let val: u64 = match operator {
        '+' => values.iter().fold(0, |acc, &x| acc + x as u64),
        '*' => values.iter().fold(1, |acc, &x| acc * x as u64),
        _ => unreachable!("Unexpected operator in part2"),
    };

    val
}

fn part2(operations_lines: &[Vec<char>]) -> u64 {
    let mut operator = '#';
    let mut op_res = 0;
    let mut operator_line = true;
    let mut values_line: Vec<u32> = vec![];

    for op_line in operations_lines {
        let mut val = 0;

        if op_line.iter().all(|&c| c == ' ') {
            op_res += calc(operator, &values_line);
            operator_line = true;
            values_line.clear();
            continue;
        }

        // L'operateur est toujours le dernier char de la ligne
        if operator_line {
            operator = *op_line.last().unwrap();
            operator_line = false;
        }

        for &c in op_line {
            // On lit les digits du haut vers le bas (du plus significatif au moins significatif) :
            // val = val*10 + d fonctionne car au 1er digit val = 0, donc le *10 est indolore.
            if let Some(d) = c.to_digit(10) {
                val = val * 10 + d;
            }
        }
        values_line.push(val);
    }

    // Add last line result
    op_res += calc(operator, &values_line);

    op_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        assert_eq!(part1(&parse_p1(example_input)), 4277556);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        assert_eq!(part2(&parse_p2(example_input)), 3263827);
    }
}
