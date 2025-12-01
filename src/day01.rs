pub fn part1(input: &str) -> i32 {
    let mut offset = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let dir = &line[0..1];
        let value: i32 = match line[1..].parse() {
            Ok(v) => v,
            Err(_) => panic!("Failed to parse integer from line: {}", &line[1..]),
        };

        if dir == "L" {
            offset -= value;
        } else {
            offset += value;
        }

        offset = offset % 100;

        if offset == 0 {
            zeroes += 1;
        }
    }

    zeroes
}

pub fn part2(input: &str) -> i32 {
    let mut offset = 50;
    let mut zeroes = 0;

    for line in input.lines() {
        let dir = &line[0..1];
        let mut value: i32 = match line[1..].parse() {
            Ok(v) => v,
            Err(_) => panic!("Failed to parse integer from line: {}", &line[1..]),
        };

        while value > 0 {
            if dir == "L" {
                offset -= 1;
                value -= 1;
            } else {
                offset += 1;
                value -= 1;
            }

            offset = offset % 100;

            if offset == 0 {
                zeroes += 1;
            }
        }
    }

    zeroes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = std::fs::read_to_string("day01/example1.txt").expect("Example file not found");
        assert_eq!(3, part1(&input));
    }

    #[test]
    fn test_part2_example() {
        let input = std::fs::read_to_string("day01/example2.txt").expect("Example file not found");
        assert_eq!(6, part2(&input));
    }

    #[test]
    fn test_part1_input() {
        let input = std::fs::read_to_string("day01/input.txt").expect("Input file not found");
        println!("Part 1: {}", part1(&input));
    }

    #[test]
    fn test_part2_input() {
        let input = std::fs::read_to_string("day01/input.txt").expect("Input file not found");
        println!("Part 2: {}", part2(&input));
    }
}
