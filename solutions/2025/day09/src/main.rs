fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../example.txt");

    let tiles = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&tiles));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&tiles));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn area(p1: (usize, usize), p2: (usize, usize)) -> usize {
    (p1.0.abs_diff(p2.0) + 1) * (p1.1.abs_diff(p2.1) + 1)
}

fn part1(tiles: &[(usize, usize)]) -> usize {
    let mut max = 0;

    for (i, &p1) in tiles.iter().enumerate() {
        for &p2 in &tiles[i + 1..] {
            let area = area(p1, p2);
            max = max.max(area);
        }
    }

    max
}

// Specific data structures for part 2

#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct Segment {
    a: Point,
    b: Point,
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    x_min: usize,
    x_max: usize,
    y_min: usize,
    y_max: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)
    }

    fn center(&self) -> Point {
        Point {
            x: (self.x_min + self.x_max) / 2,
            y: (self.y_min + self.y_max) / 2,
        }
    }
}

fn rect_from_points(p1: Point, p2: Point) -> Rectangle {
    Rectangle {
        x_min: p1.x.min(p2.x),
        x_max: p1.x.max(p2.x),
        y_min: p1.y.min(p2.y),
        y_max: p1.y.max(p2.y),
    }
}

fn to_point(t: (usize, usize)) -> Point {
    Point { x: t.0, y: t.1 }
}

fn segments(tiles: &[(usize, usize)]) -> Vec<Segment> {
    let n = tiles.len();
    let mut segments = Vec::with_capacity(n);

    for i in 0..n {
        let a = to_point(tiles[i]);
        let b = to_point(tiles[(i + 1) % n]); // Si index i + 1 == n, on retourne à 0 pour finir la
                                              // boucle
        segments.push(Segment { a, b });
    }

    segments
}

fn segment_x_rectangle(seg: Segment, rect: Rectangle) -> bool {
    let mut a = seg.a;
    let mut b = seg.b;
    let mut res = false;

    // Vérifier si le segment est horizontal
    if a.y == b.y {
        if a.x > b.x {
            std::mem::swap(&mut a, &mut b);
        }

        // Vérifier si le segment est à l'intérieur des limites x du rectangle
        res = (a.x < rect.x_max && b.x > rect.x_min) && (a.y > rect.y_min && a.y < rect.y_max);
    }

    // Vérifier si le segment est vertical
    if a.x == b.x {
        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }

        // Vérifier si le segment est à l'intérieur des limites y du rectangle
        res = (a.y < rect.y_max && b.y > rect.y_min) && (a.x > rect.x_min && a.x < rect.x_max);
    }

    res
}

fn point_inside_polygon(p: Point, segments: &[Segment]) -> bool {
    let mut count = 0;

    for segment in segments {
        let mut a = segment.a;
        let mut b = segment.b;

        if a.y == b.y {
            continue; // Segment horizontal
        }

        if a.y > b.y {
            std::mem::swap(&mut a, &mut b);
        }

        if p.y < a.y || p.y >= b.y {
            continue;
        }

        if a.x > p.x {
            count += 1;
        }
    }

    count % 2 == 1
}

fn part2(tiles: &[(usize, usize)]) -> usize {
    let segments = segments(&tiles);
    let mut max = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let p1 = to_point(tiles[i]);
            let p2 = to_point(tiles[j]);

            let rectangle = rect_from_points(p1, p2);

            if segments.iter().any(|&s| segment_x_rectangle(s, rectangle)) {
                continue;
            }

            let center = rectangle.center();
            if !point_inside_polygon(center, &segments) {
                continue;
            }

            let area = rectangle.area();

            max = max.max(area);
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for Rectangle area calculation
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle {
            x_min: 1,
            x_max: 4,
            y_min: 2,
            y_max: 5,
        };
        assert_eq!(rect.area(), 16);
    }

    // Test for point_inside_polygon function
    #[test]
    fn test_point_inside_polygon() {
        let tiles = vec![(1, 1), (5, 1), (5, 5), (1, 5)];
        let segments = segments(&tiles);
        let inside_point = Point { x: 3, y: 3 };
        let outside_point = Point { x: 6, y: 3 };
        assert!(point_inside_polygon(inside_point, &segments));
        assert!(!point_inside_polygon(outside_point, &segments));
    }

    // Test for segment_x_rectangle function
    #[test]
    fn test_segment_x_rectangle() {
        let seg = Segment {
            a: Point { x: 1, y: 1 },
            b: Point { x: 1, y: 5 },
        };
        let rect = Rectangle {
            x_min: 0,
            x_max: 2,
            y_min: 2,
            y_max: 4,
        };
        assert!(segment_x_rectangle(seg, rect));
    }

    #[test]
    fn test_part1_example() {
        let example_input = include_str!("../example.txt");
        let tiles = parse(example_input);
        assert_eq!(part1(&tiles), 50);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let tiles = parse(example_input);
        assert_eq!(part2(&tiles), 24);
    }
}
