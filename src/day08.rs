use std::collections::{HashMap, HashSet};

struct Node {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

fn distance(first: &Node, other: &Node) -> i64 {
    let dx = (first.x - other.x).pow(2);
    let dy = (first.y - other.y).pow(2);
    let dz = (first.z - other.z).pow(2);
    (dx + dy + dz).isqrt()
}

fn parse_nodes(input: &str) -> Vec<Node> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<i64> = line
                .trim()
                .split(',')
                .filter_map(|part| part.parse::<i64>().ok())
                .collect();
            Node {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn shortest_n_distances(nodes: &Vec<Node>, n: usize) -> Vec<(usize, usize)> {
    let mut indices: Vec<(usize, usize)> = Vec::new();
    let mut distances = Vec::new();
    for (i, node) in nodes.iter().enumerate() {
        for (j, other) in nodes.iter().enumerate().skip(i) {
            if i == j {
                continue;
            }
            let dist = distance(node, other);
            // Insert in distance order
            let pos = distances.binary_search(&dist).unwrap_or_else(|e| e);
            distances.insert(pos, dist);
            indices.insert(pos, (i, j));

            // Keep only the shortest n distances
            if distances.len() > n {
                distances.pop();
                indices.pop();
            }
        }
    }
    indices
}

fn connect_n_nodes(shortest: Vec<(usize, usize)>, n: usize) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for i in 0..n {
        let mut set = HashSet::new();
        set.insert(i);
        circuits.push(set);
    }

    let mut shortest = shortest.clone();

    while shortest.len() > 0 {
        let connection = shortest.remove(0);

        let a = circuits
            .iter()
            .position(|circuit| circuit.contains(&connection.0))
            .expect("Circuit a not found");

        let b = circuits
            .iter()
            .position(|circuit| circuit.contains(&connection.1))
            .expect("Circuit b not found");

        if a == b {
            continue;
        }

        // Merge circuits
        let circuit_b = circuits[b].clone();
        circuits[a].extend(&circuit_b);
        circuits.remove(b);
    }

    circuits
}

pub fn part1(input: &str, n: usize) -> i32 {
    let nodes = parse_nodes(input);
    let shortest = shortest_n_distances(&nodes, n);
    let mut circuits = connect_n_nodes(shortest, nodes.len());
    circuits.sort_by_key(|c| -(c.len() as i32));
    circuits.iter().take(3).map(|c| c.len() as i32).product()
}

pub fn part2(input: &str) -> i32 {
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day08/example1.txt").expect("Example file not found");
        assert_eq!(40, part1(&input, 10));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day08/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input, 1000));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day08/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day08/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
