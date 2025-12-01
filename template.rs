use std::fs;

pub fn part1(input: &str) -> String {
    // TODO: Implement part 1
    todo!("Part 1 not yet implemented")
}

pub fn part2(input: &str) -> String {
    // TODO: Implement part 2
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
TODO: Add example input here
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE), "TODO: expected result");
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE), "TODO: expected result");
    }

    #[test]
    #[ignore]
    fn test_part1_input() {
        let input = fs::read_to_string("dayXX/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = fs::read_to_string("dayXX/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
