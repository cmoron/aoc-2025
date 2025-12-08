use std::fmt;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../example.txt");
    let boxes = parse(input);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&boxes, 1000));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(&boxes));
    println!("Time: {:.4}ms", start.elapsed().as_secs_f64() * 1000.0);
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    distance: usize,
    node1: usize, // node1 index
    node2: usize, // node2 index
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Edge(nodes: {} ↔ {}, dist²: {})",
            self.node1, self.node2, self.distance
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    x: usize,
    y: usize,
    z: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Node {
    fn distance2(&self, other: &Node) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

fn parse(input: &str) -> Vec<Node> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');

            Node {
                x: parts.next().unwrap().parse().unwrap(),
                y: parts.next().unwrap().parse().unwrap(),
                z: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

fn build_edges(nodes: &[Node]) -> Vec<Edge> {
    let mut edges = vec![];

    let len = nodes.len();
    for i in 0..len {
        for j in i + 1..len {
            edges.push(Edge {
                distance: nodes[i].distance2(&nodes[j]),
                node1: i,
                node2: j,
            });
        }
    }
    edges
}

fn get_components(n: usize, adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut visited = vec![false; n];
    let mut components = Vec::new();

    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut comp = Vec::new();
        let mut stack = vec![start];
        visited[start] = true;

        while let Some(u) = stack.pop() {
            comp.push(u);

            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
        components.push(comp);
    }
    components
}

fn part1(nodes: &[Node], k: usize) -> usize {
    let mut edges = build_edges(nodes);
    edges.sort_unstable_by_key(|e| e.distance);

    let top_edges = &edges[..k];

    let mut adj_list = vec![Vec::new(); nodes.len()];

    for e in top_edges {
        adj_list[e.node1].push(e.node2);
        adj_list[e.node2].push(e.node1);
    }

    let components = get_components(nodes.len(), &adj_list);
    let mut comp_sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
    comp_sizes.sort_unstable();
    let n = comp_sizes.len() - 3;

    comp_sizes[n..].iter().fold(1, |acc, i| acc * i)
}

fn part2(nodes: &[Node]) -> usize {
    let mut res = 0;
    let mut edges = build_edges(nodes);

    edges.sort_unstable_by_key(|e| e.distance);

    let mut adj_list = vec![Vec::new(); nodes.len()];

    for e in edges {
        adj_list[e.node1].push(e.node2);
        adj_list[e.node2].push(e.node1);
        let components = get_components(nodes.len(), &adj_list);
        if components.len() == 1 {
            res = nodes[e.node1].x * nodes[e.node2].x;
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
        let example_input = include_str!("../example.txt");
        let boxes = parse(example_input);
        assert_eq!(part1(&boxes, 10), 40);
    }

    #[test]
    fn test_part2_example() {
        let example_input = include_str!("../example.txt");
        let boxes = parse(example_input);
        assert_eq!(part2(&boxes), 25272);
    }
}
