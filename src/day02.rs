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
            if left == right {
                matching_numbers.push(current);
            }

            current += 1;
        }
    }

    matching_numbers.iter().sum()
}

pub fn part2(input: &str) -> i64 {
    let mut matching_numbers = Vec::<i64>::new();

    // Input format: "11-22,33-44,55-66,..."
    for substr in input.trim().split(",") {
        let range_parts: Vec<&str> = substr.split('-').collect();
        let start: i64 = range_parts[0].trim().parse().unwrap();
        let end: i64 = range_parts[1].trim().parse().unwrap();

        // println!("Processing range {}-{}", start, end);

        let mut current = start;
        while current <= end {
            let current_str = current.to_string();
            let length = current_str.len();

            // println!("Checking number {}", current);

            for group_length in 1..=length / 2 {
                if length % group_length != 0 && group_length != 1 {
                    continue;
                }

                // println!("Checking number {} with group length {}", current, group_length);

                let mut all_match = true;
                let first_group = &current_str[0..group_length];
                for i in (group_length..length).step_by(group_length) {
                    let next_group = &current_str[i..i + group_length];
                    if first_group != next_group {
                        all_match = false;
                        break;
                    }
                }

                if all_match {
                    matching_numbers.push(current);
                    break;
                }
            }

            current += 1;
        }
    }

    matching_numbers.iter().sum()
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
    fn test_part2_example() {
        let input = std::fs::read_to_string("day02/example1.txt").expect("Example file not found");
        assert_eq!(4174379265, part2(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day02/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day02/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
