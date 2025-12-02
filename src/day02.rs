use std::fs;

pub fn part1(input: &str) -> i64 {
    let mut matching_numbers = Vec::<i64>::new();

    // Input format: "11-22,33-44,55-66,..."
    for substr in input.trim().split(",") {
        let range_parts: Vec<&str> = substr.split('-').collect();
        let start: i64 = range_parts[0].trim().parse().unwrap();
        let end: i64 = range_parts[1].trim().parse().unwrap();

        let mut current = start;
        while current <= end {
            let current_str = current.to_string();
            let length = current_str.len();
            if length % 2 != 0 {
                current += 1;
                continue;
            }

            let (left, right) = current_str.split_at(length / 2);
            if (left == right) {
                matching_numbers.push(current);
            }

            current += 1;
        }
    }

    matching_numbers.iter().sum()
}

pub fn part2(input: &str) -> i64 {
    todo!("Part 2 not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day02/example1.txt").expect("Example file not found");
        assert_eq!(1227775554, part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day02/example2.txt").expect("Example file not found");
        todo!();
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = fs::read_to_string("day02/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    #[ignore]
    fn test_part2_input() {
        let input = fs::read_to_string("day02/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
