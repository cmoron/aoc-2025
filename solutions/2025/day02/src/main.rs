fn main() {
    let input = include_str!("../input.txt");

    let ranges = input.trim().split(',').collect::<Vec<&str>>();

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&ranges));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&ranges));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn part1(ranges: &[&str]) -> i64 {
    let mut res = 0;

    for range in ranges {
        let (id1_str, id2_str) = range.split_once('-').unwrap();
        let id1 = id1_str.parse::<i64>().expect("Parsing error {id2_str}");
        let id2 = id2_str.parse::<i64>().expect("Parsing error {id2_str}");

        for id in id1..=id2 {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let (start, end) = id_str.split_at(id_str.len() / 2);
                if start == end {
                    res += id_str.parse::<i64>().unwrap();
                }
            }
        }
    }
    res
}

fn part2(ranges: &[&str]) -> i64 {
    let mut res = 0;

    for range in ranges {
        let (id1_str, id2_str) = range.split_once('-').unwrap();
        let id1 = id1_str.parse::<i64>().expect("Parsing error {id2_str}");
        let id2 = id2_str.parse::<i64>().expect("Parsing error {id2_str}");

        for id in id1..=id2 {
            let id_str = id.to_string();

            for cut in 1..=(id_str.len() / 2) {
                if id_str.len() % cut == 0 {
                    let parts : Vec<&str> = id_str.as_bytes()
                        .chunks(cut)
                        .map(|chunk| std::str::from_utf8(chunk).unwrap())
                        .collect();

                    let first = parts.iter().next().unwrap();
                    if parts.iter().all(|s| s == first) {
                        res += id_str.parse::<i64>().unwrap();
                        break
                    }
                }
            }
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
        let ranges = example_input.trim().split(',').collect::<Vec<&str>>();
        assert_eq!(part1(&ranges), 1227775554);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let ranges = example_input.trim().split(',').collect::<Vec<&str>>();
        assert_eq!(part2(&ranges), 4174379265);
    }
}
