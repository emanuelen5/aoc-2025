use std::collections::HashMap as Map;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<usize>, Vec<HashSet<usize>>) {
    let beam = input
        .lines()
        .next()
        .and_then(|line| line.find('S'))
        .expect("Failed to parse start offset");
    let beams = HashSet::from([beam]);
    // Find the index of ^ in each subsequent line
    let splitters: Vec<HashSet<usize>> = input
        .lines()
        .skip(1)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect()
        })
        .collect();

    (beams, splitters)
}

fn split_beams(beams: &HashSet<usize>, splitters: &HashSet<usize>) -> (HashSet<usize>, usize) {
    let mut new_beams = HashSet::new();
    let mut splits = 0;
    for beam in beams {
        if splitters.contains(beam) {
            splits += 1;
            new_beams.insert(beam - 1);
            new_beams.insert(beam + 1);
        } else {
            new_beams.insert(*beam);
        }
    }

    (new_beams, splits)
}

fn split_beams_with_count(
    beams: &Map<usize, usize>,
    splitters: &HashSet<usize>,
) -> Map<usize, usize> {
    let mut new_beams: Map<usize, usize> = Map::new();
    for (beam, count) in beams {
        if splitters.contains(beam) {
            *new_beams.entry(beam - 1).or_insert(0) += count;
            *new_beams.entry(beam + 1).or_insert(0) += count;
        } else {
            *new_beams.entry(*beam).or_insert(0) += count;
        }
    }

    new_beams
}

pub fn part1(input: &str) -> usize {
    let (mut beams, splitters) = parse_input(input);
    let mut total_splits = 0;

    for i in 0..splitters.len() {
        let (new_beams, splits) = split_beams(&beams, &splitters[i]);
        beams = new_beams;
        total_splits += splits;
    }

    total_splits
}

pub fn part2(input: &str) -> usize {
    let (start_beams, splitters) = parse_input(input);
    let mut beams: Map<usize, usize> = Map::new();
    for beam in start_beams {
        *beams.entry(beam).or_insert(0) += 1;
    }

    for splitter in splitters {
        beams = split_beams_with_count(&beams, &splitter);
    }

    let total = beams.iter().map(|(_, count)| count).sum();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("inputs/day07/example1.txt").expect("Example file not found");
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("inputs/day07/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("inputs/day07/example1.txt").expect("Example file not found");
        assert_eq!(40, part2(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("inputs/day07/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
