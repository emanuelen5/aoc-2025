pub fn part1(_input: &str) -> i32 {
    todo!("Part 1 not yet implemented")
}

pub fn part2(_input: &str) -> i32 {
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("dayXX/example1.txt").expect("Example file not found");
        assert_eq!(3, part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part1_input() {
        let input = std::fs::read_to_string("dayXX/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("dayXX/example1.txt").expect("Example file not found");
        assert_eq!(3, part2(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = std::fs::read_to_string("dayXX/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
