use std::collections::HashMap;
use std::fmt;

fn get_neighbors<T: Copy>(matrix: &[Vec<T>], row: usize, col: usize) -> HashMap<&'static str, T> {
    let rows = matrix.len() as isize;
    let cols = matrix[0].len() as isize;

    let directions: [(&str, (isize, isize)); 8] = [
        ("N", (-1, 0)),
        ("NE", (-1, 1)),
        ("E", (0, 1)),
        ("SE", (1, 1)),
        ("S", (1, 0)),
        ("SW", (1, -1)),
        ("W", (0, -1)),
        ("NW", (-1, -1)),
    ];

    let mut neighbors = HashMap::new();

    for (dir, (dr, dc)) in directions {
        let nr = row as isize + dr;
        let nc = col as isize + dc;

        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            neighbors.insert(dir, matrix[nr as usize][nc as usize]);
        }
    }

    neighbors
}

struct Diagram {
    grid: Vec<Vec<char>>,
}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut diagram = Diagram { grid: parse(input) };

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&diagram));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&mut diagram));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part1(diagram: &Diagram) -> usize {
    let mut res = 0;
    for i in 0..diagram.grid.len() {
        for j in 0..diagram.grid[i].len() {
            if diagram.grid[i][j] == '@' {
                let neighbors = get_neighbors(&diagram.grid, i, j);
                let nrolls = neighbors.values().filter(|&&v| v == '@').count();
                if nrolls < 4 {
                    res += 1;
                }
            }
        }
    }
    res
}

fn part2(diagram: &mut Diagram) -> usize {
    let mut res = 0;
    loop {
        let mut found = false;
        for i in 0..diagram.grid.len() {
            for j in 0..diagram.grid[i].len() {
                if diagram.grid[i][j] == '@' {
                    let neighbors = get_neighbors(&diagram.grid, i, j);
                    let nrolls = neighbors.values().filter(|&&v| v == '@').count();
                    if nrolls < 4 {
                        diagram.grid[i][j] = 'x';
                        res += 1;
                        found = true;
                    }
                }
            }
        }
        if !found {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../example.txt");
        let diagram = Diagram { grid: parse(input) };
        assert_eq!(part1(&diagram), 13);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../example.txt");
        let mut diagram = Diagram { grid: parse(input) };
        assert_eq!(part2(&mut diagram), 43);
    }
}
