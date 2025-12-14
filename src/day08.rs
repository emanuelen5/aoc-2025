use std::collections::HashSet;

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
    let mut pairs: Vec<(i64, usize, usize)> = Vec::new();

    for (i, node) in nodes.iter().enumerate() {
        for (j, other) in nodes.iter().enumerate().skip(i + 1) {
            let dist = distance(node, other);
            pairs.push((dist, i, j));
        }
    }

    pairs.sort_unstable_by_key(|&(dist, _, _)| dist);
    pairs.into_iter().take(n).map(|(_, i, j)| (i, j)).collect()
}

fn create_initial_circuits(n: usize) -> Vec<HashSet<usize>> {
    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for i in 0..n {
        let mut set = HashSet::new();
        set.insert(i);
        circuits.push(set);
    }
    circuits
}

fn connect_and_merge_circuits(circuits: &mut Vec<HashSet<usize>>, connection: (usize, usize)) {
    let a = circuits
        .iter()
        .position(|circuit| circuit.contains(&connection.0))
        .expect("Circuit a not found");

    let b = circuits
        .iter()
        .position(|circuit| circuit.contains(&connection.1))
        .expect("Circuit b not found");

    if a == b {
        return;
    }

    // Merge circuits
    let circuit_b = circuits[b].clone();
    circuits[a].extend(&circuit_b);
    circuits.remove(b);
}

fn connect_n_nodes(shortest: Vec<(usize, usize)>, n: usize) -> Vec<HashSet<usize>> {
    let mut circuits = create_initial_circuits(n);

    let mut shortest = shortest.clone();

    while shortest.len() > 0 {
        let connection = shortest.remove(0);

        connect_and_merge_circuits(&mut circuits, connection);
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

pub fn part2(input: &str) -> i64 {
    let nodes = parse_nodes(input);
    let n = nodes.len() * nodes.len() / 2 - nodes.len();
    let connections = shortest_n_distances(&nodes, n);
    let mut circuits = create_initial_circuits(nodes.len());

    let mut final_connection: (usize, usize) = (0, 0);
    for conn in connections.iter() {
        connect_and_merge_circuits(&mut circuits, *conn);

        let are_done = circuits.len() == 1;
        if are_done {
            final_connection = *conn;
            break;
        }
    }

    let node1 = &nodes[final_connection.0];
    let node2 = &nodes[final_connection.1];
    (node1.x * node2.x) as i64
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
    fn test_part2_example() {
        let input = std::fs::read_to_string("day08/example1.txt").expect("Example file not found");
        assert_eq!(25272, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day08/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
