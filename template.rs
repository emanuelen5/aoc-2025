use std::fs;

pub fn part1(input: &str) -> String {
    todo!("Part 1 not yet implemented")
}

pub fn part2(input: &str) -> String {
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
    fn test_part2_example() {
        let input = std::fs::read_to_string("dayXX/example2.txt").expect("Example file not found");
        todo!();
        assert_eq!(3, part1(&input));
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
