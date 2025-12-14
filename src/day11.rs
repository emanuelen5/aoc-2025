use std::collections::HashMap;

struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

fn parse_input(input: &str) -> Graph<'_> {
    let mut graph = Graph {
        edges: HashMap::new(),
    };

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let key = parts[0].trim_end_matches(":");
        let values = parts[1..].to_vec();
        graph.edges.insert(key, values);
    }

    graph
}

fn path_count(graph: &Graph, start: &str) -> usize {
    let outputs = match graph.edges.get(start) {
        Some(v) => v,
        None => panic!("No edges for node {}", start),
    };

    if outputs.len() == 1 && outputs[0] == "out" {
        return 1
    }

    let mut total = 0;
    for &node in outputs {
        total += path_count(graph, node);
    }

    total
}

pub fn part1(input: &str) -> usize {
    let graph = parse_input(input);
    let count = path_count(&graph, "you");
    count
}

pub fn part2(_input: &str) -> usize {
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day11/example1.txt").expect("Example file not found");
        assert_eq!(5, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day11/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day11/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day11/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
