fn main() {
    let input = include_str!("../input.txt");

    let splits = input.lines().map(|line| line.split_at(1)).collect::<Vec<(&str, &str)>>();

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&splits));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&splits));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn part1(splits: &[(&str, &str)]) -> i32 {
    let mut res = 0;
    let mut index = 50;

    for (dir_str, dist_str) in splits {
        let dir = dir_str.parse::<char>().expect("Parse failed");
        let mut dist = dist_str.parse::<i32>().expect("Parse failed");

        if dir == 'L' {
            dist = -dist;
        }

        index += dist;

        if index.rem_euclid(100) == 0 {
            res += 1;
        }
    }

    res
}

fn part2(splits: &[(&str, &str)]) -> i32 {
    let mut res = 0;
    let mut index = 50;

    for (dir_str, dist_str) in splits {
        let dir = dir_str.parse::<char>().expect("Parse failed");
        let dist = dist_str.parse::<i32>().expect("Parse failed");

        for _ in 0..dist {
            match dir {
                'L' => index = (index - 1 + 100) % 100,
                'R' => index = (index + 1) % 100,
                _ => println!("Parse error"),
            }

            if index == 0 {
                res += 1;
            }
        }
    }

    res
}
