fn main() {
    let input = include_str!("../input.txt");
    let (presents, regions) = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&presents, &regions));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

struct Present {
    area: usize,
}

#[derive(Debug)]
struct Region {
    cols: usize,
    rows: usize,
    constraints: Vec<u8>,
}

fn area(shape_str: &str) -> usize {
    shape_str
        .lines()
        .flat_map(|line| line.chars())
        .filter(|&c| c == '#')
        .count()
}

fn parse(input: &str) -> (Vec<Present>, Vec<Region>) {
    let mut splits: Vec<&str> = input.split("\n\n").collect();

    let constraints_str: Vec<&str> = splits.pop().unwrap().lines().collect();
    let presents_str = splits;

    let presents = presents_str
        .iter()
        .map(|p| {
            let (_id, shape) = p.split_once(":\n").unwrap();
            let area = area(shape.trim());

            Present { area: area }
        })
        .collect();

    let constraints = constraints_str
        .iter()
        .map(|c| {
            let (p1, p2) = c.split_once(": ").unwrap();
            let (col, row) = p1.split_once("x").unwrap();
            let col = col.parse::<usize>().unwrap();
            let row = row.parse::<usize>().unwrap();
            let constraints: Vec<u8> = p2
                .split_whitespace()
                .map(|v| v.parse::<u8>().unwrap())
                .collect();
            Region {
                cols: col,
                rows: row,
                constraints: constraints,
            }
        })
        .collect();

    (presents, constraints)
}

fn part1(presents: &[Present], regions: &[Region]) -> usize {
    regions
        .iter()
        .filter(|r| {
            let needed: usize = r
                .constraints
                .iter()
                .zip(presents.iter())
                .map(|(&constraint, present)| constraint as usize * present.area)
                .sum();

            r.cols * r.rows >= needed
        })
        .count()
}

fn part2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        let (presents, regions) = parse(example_input);
        assert_eq!(part1(&presents, &regions), 2);
    }
}
